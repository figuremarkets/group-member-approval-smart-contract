package tech.figure.approval.member.group.devtools.client.model

import tech.figure.approval.member.group.devtools.client.model.GroupMemberContractWasmLocation.GitHub

/**
 * Various options on how to instantiate the contract.
 */
sealed interface GroupMemberContractInstantiationMode {
    /**
     * First, stores the contract on chain, and then instantiates it, fully automating the process.
     *
     * @param wasmLocation The method of fetching the contract's wasm file.
     */
    class StoreAndInstantiate(val wasmLocation: GroupMemberContractWasmLocation = GitHub()) : GroupMemberContractInstantiationMode

    /**
     * Only instantiates the contract, with the assumption that the contract has already been stored on chain.
     *
     * @param codeId The code identifier generated when the contract was stored.
     */
    class InstantiateOnly(val codeId: Long) : GroupMemberContractInstantiationMode
}
