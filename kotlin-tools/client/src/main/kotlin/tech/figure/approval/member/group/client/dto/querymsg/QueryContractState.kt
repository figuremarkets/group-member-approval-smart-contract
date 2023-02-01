package tech.figure.approval.member.group.client.dto.querymsg

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import tech.figure.approval.member.group.client.dto.querymsg.base.GroupMemberContractQuery

@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("query_contract_state")
object QueryContractState : GroupMemberContractQuery

@JsonNaming(SnakeCaseStrategy::class)
data class GroupMemberContractState(
    val admin: String,
    val attributeName: String,
    val contractName: String,
    val contractType: String,
    val contractVersion: String,
)
