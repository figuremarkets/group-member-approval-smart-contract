package tech.figure.approval.member.group.client.dto.executemsg

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import com.fasterxml.jackson.databind.annotation.JsonSerialize
import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastTxResponse
import tech.figure.approval.member.group.client.dto.executemsg.base.GroupMemberContractExecute
import tech.figure.approval.member.group.client.serialization.CosmWasmLongToUint64Serializer

@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("approve_group_membership")
data class ExecuteApproveGroupMembership(
    @JsonSerialize(using = CosmWasmLongToUint64Serializer::class)
    val groupId: Long,
) : GroupMemberContractExecute

data class ApproveGroupMembershipResponse(
    val txResponse: BroadcastTxResponse,
    val action: String,
    val accountAddress: String,
    val attributeName: String,
    val groupId: Long,
)
