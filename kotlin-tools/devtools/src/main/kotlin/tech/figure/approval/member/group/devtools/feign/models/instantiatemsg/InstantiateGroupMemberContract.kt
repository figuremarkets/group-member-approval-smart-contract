package tech.figure.approval.member.group.devtools.feign.models.instantiatemsg

import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import tech.figure.approval.member.group.client.dto.executemsg.base.GroupMemberContractExecute

/**
 * A message payload that instantiates a new instance of the group-member-approval-smart-contract.
 *
 * @param contractName This name will identify the contract when separate instances are deployed on chain.
 * @param attributeName The Provenance Blockchain Name Module name that will be bound to the contract.  When the
 * approveGroupMembership execution route is invoked, a Provenance Blockchain Attribute will be bound to the invoking
 * account that includes the specified group id as its INT value.
 * @param bindAttributeName If specified as true, the value specified in attributeName will be automatically bound
 * as a Provenance Blockchain Name to the contract during the instantiation process.  If this is omitted, the same name
 * specified in attributeName must manually be bound after the contract is instantiated.  This is useful in circumstances
 * where the name will be bound using a restricted name module namespace.
 */
@JsonNaming(SnakeCaseStrategy::class)
data class InstantiateGroupMemberContract(
    val contractName: String,
    val attributeName: String,
    val bindAttributeName: Boolean,
) : GroupMemberContractExecute

/**
 * A response payload given after contract instantiation.
 *
 * @param storedCodeId The code identifier that was assigned on chain to the code after the wasm was stored.
 * @param contractAddress The bech32 address generated for the contract instance, which can later be used to invoke it.
 */
data class InstantiateGroupMemberContractResponse(
    val storedCodeId: Long,
    val contractAddress: String,
)
