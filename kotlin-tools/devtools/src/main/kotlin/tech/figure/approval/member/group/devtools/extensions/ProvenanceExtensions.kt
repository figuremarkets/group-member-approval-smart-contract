package tech.figure.approval.member.group.devtools.extensions

import cosmos.tx.v1beta1.ServiceOuterClass

internal fun ServiceOuterClass.BroadcastTxResponse.checkSuccess(): ServiceOuterClass.BroadcastTxResponse = this.also {
    check(it.txResponse.code == 0) { "Expected msg broadcast to succeed, but failed with logs: ${it.txResponse.rawLog}" }
}
