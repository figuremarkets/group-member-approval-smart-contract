package tech.figure.approval.member.group.client.client

import com.fasterxml.jackson.databind.ObjectMapper
import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastMode
import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastTxResponse
import cosmwasm.wasm.v1.QueryOuterClass.QuerySmartContractStateRequest
import cosmwasm.wasm.v1.Tx.MsgExecuteContract
import io.provenance.client.grpc.BaseReqSigner
import io.provenance.client.grpc.PbClient
import io.provenance.client.grpc.Signer
import io.provenance.client.protobuf.extensions.queryWasm
import io.provenance.client.protobuf.extensions.toAny
import io.provenance.client.protobuf.extensions.toTxBody
import io.provenance.scope.util.toByteString
import tech.figure.approval.member.group.client.dto.executemsg.ApproveGroupMembershipResponse
import tech.figure.approval.member.group.client.dto.executemsg.ExecuteApproveGroupMembership
import tech.figure.approval.member.group.client.dto.executemsg.base.GroupMemberContractExecute
import tech.figure.approval.member.group.client.dto.querymsg.GroupMemberContractState
import tech.figure.approval.member.group.client.dto.querymsg.QueryContractState
import tech.figure.approval.member.group.client.dto.querymsg.base.GroupMemberContractQuery
import tech.figure.approval.member.group.client.extensions.checkSuccess
import tech.figure.approval.member.group.client.extensions.getAttributeValue
import tech.figure.approval.member.group.client.extensions.singleWasmEvent
import tech.figure.approval.member.group.client.util.GroupMemberApprovalOMUtil
import tech.figure.approval.member.group.client.util.GroupMemberContractAddressResolver
import tendermint.abci.Types.Event

open class GroupMemberContractClient(
    protected val pbClient: PbClient,
    private val addressResolver: GroupMemberContractAddressResolver,
    protected val objectMapper: ObjectMapper = GroupMemberApprovalOMUtil.getObjectMapper(),
) {
    val contractAddress by lazy { addressResolver.getAddress(pbClient) }

    fun executeGroupMemberApproval(
        executeMsg: ExecuteApproveGroupMembership,
        signer: Signer,
        broadcastMode: BroadcastMode = BroadcastMode.BROADCAST_MODE_BLOCK,
    ): ApproveGroupMembershipResponse = executeContract(
        executeMsg = executeMsg,
        signer = signer,
        broadcastMode = broadcastMode,
    ).let { (event, txResponse) ->
        ApproveGroupMembershipResponse(
            txResponse = txResponse,
            action = event.getAttributeValue(attribute = "action"),
            accountAddress = event.getAttributeValue(attribute = "account_address"),
            attributeName = event.getAttributeValue(attribute = "attribute_name"),
            groupId = event.getAttributeValue(attribute = "group_id").toLongOrNull()
                ?: error("expected group_id attribute to be emitted by the smart contract")
        )
    }

    fun queryContractState(): GroupMemberContractState = queryContract(QueryContractState)

    protected fun executeContract(
        executeMsg: GroupMemberContractExecute,
        signer: Signer,
        broadcastMode: BroadcastMode,
    ): Pair<Event, BroadcastTxResponse> = pbClient.estimateAndBroadcastTx(
        txBody = MsgExecuteContract.newBuilder().also { msg ->
            msg.msg = objectMapper.writeValueAsString(executeMsg).toByteString()
            msg.contract = contractAddress
            msg.sender = signer.address()
        }.build().toAny().toTxBody(),
        signers = BaseReqSigner(signer = signer).let(::listOf),
        mode = broadcastMode,
    ).checkSuccess().let { response -> response.singleWasmEvent() to response }

    private inline fun <T : GroupMemberContractQuery, reified U : Any> queryContract(
        queryMsg: T,
    ): U = pbClient.wasmClient.queryWasm(
        QuerySmartContractStateRequest.newBuilder().also { req ->
            req.address = contractAddress
            req.queryData = objectMapper.writeValueAsString(queryMsg).toByteString()
        }.build()
    ).data.toByteArray().let { objectMapper.readValue(it, U::class.java) }
}
