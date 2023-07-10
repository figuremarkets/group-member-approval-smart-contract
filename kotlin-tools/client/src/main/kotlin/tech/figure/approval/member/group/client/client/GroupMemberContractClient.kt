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

/**
 * This client file is designed to communicate with the group-member-approval-smart contract.  It mirrors all functionality
 * exposed by each execution and query route on the contract.
 */
open class GroupMemberContractClient(
    protected val pbClient: PbClient,
    private val addressResolver: GroupMemberContractAddressResolver,
    protected val objectMapper: ObjectMapper = GroupMemberApprovalOMUtil.getObjectMapper(),
) {
    val contractAddress by lazy { addressResolver.getAddress(pbClient) }

    /**
     * Sends a msg to the contract to signify that the signing address has consented to be a member of the given group.
     */
    fun executeGroupMemberApproval(
        executeMsg: ExecuteApproveGroupMembership,
        signer: Signer,
        broadcastMode: BroadcastMode = BroadcastMode.BROADCAST_MODE_BLOCK,
        feeGranter: String? = null,
    ): ApproveGroupMembershipResponse = executeContract(
        executeMsg = executeMsg,
        signer = signer,
        broadcastMode = broadcastMode,
        feeGranter = feeGranter,
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

    /**
     * Generates a msg that signifies that the expected signer has consented to be a member of the given group.  This
     * route is useful for generating a proper msg format without immediately signing it, allowing the transaction to
     * be passed to an external entity, or for the msg to be batched into a larger transaction's set of messages.
     */
    fun genExecuteGroupMemberApprovalMsg(
        executeMsg: ExecuteApproveGroupMembership,
        signerAddress: String,
    ): MsgExecuteContract = genMsg(executeMsg = executeMsg, signerAddress = signerAddress)

    /**
     * Returns all details pertaining to the smart contract's state, which denotes its name, version, etc.
     */
    fun queryContractState(): GroupMemberContractState = queryContract(QueryContractState)

    private fun genMsg(
        executeMsg: GroupMemberContractExecute,
        signerAddress: String,
    ): MsgExecuteContract = MsgExecuteContract.newBuilder().also { msg ->
        msg.msg = objectMapper.writeValueAsString(executeMsg).toByteString()
        msg.contract = contractAddress
        msg.sender = signerAddress
    }.build()

    private fun executeContract(
        executeMsg: GroupMemberContractExecute,
        signer: Signer,
        broadcastMode: BroadcastMode,
        feeGranter: String?,
    ): Pair<Event, BroadcastTxResponse> = pbClient.estimateAndBroadcastTx(
        txBody = genMsg(executeMsg = executeMsg, signerAddress = signer.address()).toAny().toTxBody(),
        signers = BaseReqSigner(signer = signer).let(::listOf),
        mode = broadcastMode,
        feeGranter = feeGranter,
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
