package tech.figure.approval.member.group.client.dto.executemsg

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import com.fasterxml.jackson.databind.annotation.JsonSerialize
import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastTxResponse
import tech.figure.approval.member.group.client.dto.executemsg.base.GroupMemberContractExecute
import tech.figure.approval.member.group.client.serialization.CosmWasmLongToUint64Serializer

/**
 * A payload to allow a group member to approve of its membership.
 *
 * @param groupId The unique Provenance Blockchain Group identifier for which the member will consent to membership.
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("approve_group_membership")
data class ExecuteApproveGroupMembership(
    @JsonSerialize(using = CosmWasmLongToUint64Serializer::class)
    val groupId: Long,
) : GroupMemberContractExecute

/**
 * The response values from an executed group membership approval msg.
 *
 * @param txResponse The raw blockchain transaction response details from execution.
 * @param action The contact's specified action key for this execution route.
 * @param accountAddress The bech32 address of the account that signed the msg.
 * @param attributeName The Provenance Blockchain Name that is tied to the attribute written to the signer's account.
 * @param groupId The Provenance Blockchain Group identifier that is written into the attribute value.
 */
data class ApproveGroupMembershipResponse(
    val txResponse: BroadcastTxResponse,
    val action: String,
    val accountAddress: String,
    val attributeName: String,
    val groupId: Long,
)
