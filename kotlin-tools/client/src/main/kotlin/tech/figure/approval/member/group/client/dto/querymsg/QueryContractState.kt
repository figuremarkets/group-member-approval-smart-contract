package tech.figure.approval.member.group.client.dto.querymsg

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import tech.figure.approval.member.group.client.dto.querymsg.base.GroupMemberContractQuery

/**
 * An empty payload, which is required by the contract to fetch its state values.
 */
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("query_contract_state")
object QueryContractState : GroupMemberContractQuery

/**
 * The response returned from querying contract state values.
 *
 * @param admin The bech32 address of the account that currently has admin rights to the contract.
 * @param attributeName The Provenance Blockchain Name that is bound to the contract for attribute creation.
 * @param contractName The custom name given to the contract when it was instantiated.
 * @param contractType The hardcoded contract type value that is used for validation during migrations.
 * @param contractVersion The currently deployed version of the contract, which can be cross-referenced with the repository's release tags.
 */
@JsonNaming(SnakeCaseStrategy::class)
data class GroupMemberContractState(
    val admin: String,
    val attributeName: String,
    val contractName: String,
    val contractType: String,
    val contractVersion: String,
)
