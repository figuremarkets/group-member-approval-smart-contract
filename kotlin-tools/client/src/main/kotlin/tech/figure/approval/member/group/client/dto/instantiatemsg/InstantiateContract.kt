package tech.figure.approval.member.group.client.dto.instantiatemsg

import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming

@JsonNaming(SnakeCaseStrategy::class)
data class InstantiateContract(
    val contractName: String,
    val attributeName: String,
    val bindAttributeName: Boolean,
)
