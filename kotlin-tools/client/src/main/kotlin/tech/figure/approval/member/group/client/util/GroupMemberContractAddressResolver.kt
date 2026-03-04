package tech.figure.approval.member.group.client.util

import io.provenance.client.grpc.AbstractPbClient
import io.provenance.client.protobuf.extensions.resolveAddressForName

sealed interface GroupMemberContractAddressResolver {
    class ProvidedAddress(val address: String) : GroupMemberContractAddressResolver
    class FromName(val name: String) : GroupMemberContractAddressResolver

    fun getAddress(pbClient: AbstractPbClient<*>): String = when (this) {
        is ProvidedAddress -> address
        is FromName -> pbClient.nameClient.resolveAddressForName(name)
    }
}
