package tech.figure.approval.member.group.devtools.client

import com.fasterxml.jackson.databind.ObjectMapper
import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastMode
import cosmwasm.wasm.v1.Tx
import cosmwasm.wasm.v1.Tx.MsgStoreCode
import cosmwasm.wasm.v1.Types
import io.provenance.client.grpc.BaseReqSigner
import io.provenance.client.grpc.PbClient
import io.provenance.client.grpc.Signer
import io.provenance.client.protobuf.extensions.toAny
import io.provenance.client.protobuf.extensions.toTxBody
import io.provenance.scope.encryption.util.orThrow
import io.provenance.scope.util.toByteString
import tech.figure.approval.member.group.client.client.GroupMemberContractClient
import tech.figure.approval.member.group.client.util.GroupMemberApprovalOMUtil
import tech.figure.approval.member.group.client.util.GroupMemberContractAddressResolver
import tech.figure.approval.member.group.devtools.client.model.GroupMemberContractInstantiationMode
import tech.figure.approval.member.group.devtools.client.model.GroupMemberContractInstantiationMode.InstantiateOnly
import tech.figure.approval.member.group.devtools.client.model.GroupMemberContractInstantiationMode.StoreAndInstantiate
import tech.figure.approval.member.group.devtools.client.model.GroupMemberContractWasmLocation
import tech.figure.approval.member.group.devtools.client.model.GroupMemberContractWasmLocation.GitHub
import tech.figure.approval.member.group.devtools.client.model.GroupMemberContractWasmLocation.LocalFile
import tech.figure.approval.member.group.devtools.extensions.base64Decode
import tech.figure.approval.member.group.devtools.extensions.checkSuccess
import tech.figure.approval.member.group.devtools.extensions.gzip
import tech.figure.approval.member.group.devtools.feign.GitHubApiClient
import tech.figure.approval.member.group.devtools.feign.models.instantiatemsg.InstantiateGroupMemberContract
import tech.figure.approval.member.group.devtools.feign.models.instantiatemsg.InstantiateGroupMemberContractResponse
import java.io.File
import java.net.URL

class LocalGroupMemberContractClient(
    pbClient: PbClient,
    addressResolver: GroupMemberContractAddressResolver,
    objectMapper: ObjectMapper = GroupMemberApprovalOMUtil.getObjectMapper(),
) : GroupMemberContractClient(
    pbClient = pbClient,
    addressResolver = addressResolver,
    objectMapper = objectMapper,
) {
    private companion object {
        const val FIGURE_ORGANIZATION = "FigureTechnologies"
        const val CONTRACT_REPOSITORY = "group-member-approval-smart-contract"
    }

    fun instantiateContract(
        instantiateMsg: InstantiateGroupMemberContract,
        admin: Signer,
        instantiationMode: GroupMemberContractInstantiationMode = StoreAndInstantiate(),
        attributeName: String = "memberapproval.pb",
    ): InstantiateGroupMemberContractResponse = when (instantiationMode) {
        is StoreAndInstantiate -> storeContractGetCodeId(admin = admin, wasmLocation = instantiationMode.wasmLocation)
        is InstantiateOnly -> instantiationMode.codeId
    }.let { codeId ->
        pbClient.estimateAndBroadcastTx(
            txBody = Tx.MsgInstantiateContract.newBuilder().also { instantiate ->
                instantiate.admin = admin.address()
                instantiate.codeId = codeId
                instantiate.label = "group-member-approval"
                instantiate.sender = admin.address()
                instantiate.msg = InstantiateGroupMemberContract(
                    contractName = "group_member_approval_smart_contract",
                    attributeName = attributeName,
                    bindAttributeName = true,
                ).let(objectMapper::writeValueAsString).toByteString()
            }.build().toAny().toTxBody(),
            signers = BaseReqSigner(signer = admin).let(::listOf),
            gasAdjustment = 1.1,
        ).txResponse
            .eventsList
            .singleOrNull { it.type.base64Decode() == "instantiate" }
            ?.attributesList
            ?.singleOrNull { it.key.toStringUtf8().base64Decode() == "_contract_address" }
            ?.value
            ?.toStringUtf8()
            ?.base64Decode()
            .orThrow { IllegalStateException("Failed to find contract address after instantiating contract with code id [$codeId]") }
            .let { contractAddress ->
                InstantiateGroupMemberContractResponse(storedCodeId = codeId, contractAddress = contractAddress)
            }
    }

    private fun storeContractGetCodeId(
        admin: Signer,
        wasmLocation: GroupMemberContractWasmLocation,
    ): Long = when (wasmLocation) {
        is GitHub -> {
            GitHubApiClient.new().let { client ->
                wasmLocation.contractReleaseTag?.let { tag ->
                    client.getReleaseByTag(
                        organization = FIGURE_ORGANIZATION,
                        repository = CONTRACT_REPOSITORY,
                        tag = tag,
                    )
                } ?: client.getLatestRelease(organization = FIGURE_ORGANIZATION, repository = CONTRACT_REPOSITORY)
            }.assets
                .singleOrNull { it.name == "group_member_approval_smart_contract.wasm" }
                .orThrow { IllegalStateException("Expected the contract repository to include a wasm file for tag [${wasmLocation.contractReleaseTag ?: "latest"}]") }
                .browserDownloadUrl
                .let(::URL)
                .readBytes()
        }

        is LocalFile.AbsolutePath -> File(wasmLocation.absoluteFilePath).readBytes()
        is LocalFile.ProjectResource -> ClassLoader.getSystemResource(wasmLocation.resourcePath).file.let(::File)
            .readBytes()
    }.gzip().let { gzippedWasmBytes ->
        pbClient.estimateAndBroadcastTx(
            txBody = MsgStoreCode.newBuilder().also { storeCode ->
                storeCode.instantiatePermissionBuilder.address = admin.address()
                storeCode.instantiatePermissionBuilder.permission = Types.AccessType.ACCESS_TYPE_ONLY_ADDRESS
                storeCode.sender = admin.address()
                storeCode.wasmByteCode = gzippedWasmBytes.toByteString()
            }.build().toAny().toTxBody(),
            signers = BaseReqSigner(signer = admin).let(::listOf),
            mode = BroadcastMode.BROADCAST_MODE_BLOCK,
        ).checkSuccess()
            .txResponse
            .eventsList
            .singleOrNull { it.type.base64Decode() == "store_code" }
            ?.attributesList
            ?.singleOrNull { it.key.toStringUtf8().base64Decode() == "code_id" }
            ?.value
            ?.toStringUtf8()
            ?.base64Decode()
            ?.toLongOrNull()
            .orThrow { IllegalStateException("Failed to derive code id from stored contract") }
    }
}
