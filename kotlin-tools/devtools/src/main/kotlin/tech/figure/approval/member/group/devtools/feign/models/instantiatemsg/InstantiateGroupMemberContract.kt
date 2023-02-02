package tech.figure.approval.member.group.devtools.feign.models.instantiatemsg

import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import tech.figure.approval.member.group.client.dto.executemsg.base.GroupMemberContractExecute

@JsonNaming(SnakeCaseStrategy::class)
data class InstantiateGroupMemberContract(
    val contractName: String,
    val attributeName: String,
    val bindAttributeName: Boolean,
) : GroupMemberContractExecute

data class InstantiateGroupMemberContractResponse(
    val storedCodeId: Long,
    val contractAddress: String,
)
