package tech.figure.approval.member.group.devtools.client.model

import tech.figure.approval.member.group.devtools.client.model.GroupMemberContractWasmLocation.GitHub

sealed interface GroupMemberContractInstantiationMode {
    class StoreAndInstantiate(val wasmLocation: GroupMemberContractWasmLocation = GitHub()) : GroupMemberContractInstantiationMode
    class InstantiateOnly(val codeId: Long) : GroupMemberContractInstantiationMode
}
