package tech.figure.approval.member.group.devtools.extensions

import java.util.Base64

/**
 * Attempts to convert the value from base64 to a readable string.  If the value is not in base64 format, the original
 * value will be returned.
 */
internal fun String.base64DecodeOrValue(): String = try {
    Base64.getDecoder().decode(this).toString(Charsets.UTF_8)
} catch (e: Exception) {
    this
}
