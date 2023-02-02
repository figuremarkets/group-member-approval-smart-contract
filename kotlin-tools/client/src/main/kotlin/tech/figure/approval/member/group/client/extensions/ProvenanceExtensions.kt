package tech.figure.approval.member.group.client.extensions

import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastTxResponse
import tendermint.abci.Types.Event

internal fun BroadcastTxResponse.singleWasmEvent(): Event = txResponse
    .eventsList
    .singleOrNull { it.type == "wasm" }
    ?: error("Expected a single wasm event to be emitted when executing the smart contract")

internal fun Event.getAttributeValue(attribute: String): String = attributesList
    .singleOrNull { it.key.toStringUtf8() == attribute }
    ?.value
    ?.toStringUtf8()
    ?: error("Expected attribute [$attribute] to be emitted by the smart contract")

internal fun BroadcastTxResponse.checkSuccess(): BroadcastTxResponse = this.also {
    check(it.txResponse.code == 0) { "Expected msg broadcast to succeed, but failed with logs: ${it.txResponse.rawLog}" }
}
