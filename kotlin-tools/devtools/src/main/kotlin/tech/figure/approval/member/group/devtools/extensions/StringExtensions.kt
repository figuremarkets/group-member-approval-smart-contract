package tech.figure.approval.member.group.devtools.extensions

import java.util.Base64

internal fun String.base64Decode(): String = Base64.getDecoder().decode(this).toString(Charsets.UTF_8)
