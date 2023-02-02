package tech.figure.approval.member.group.devtools.extensions

import java.io.ByteArrayOutputStream
import java.util.zip.GZIPOutputStream

internal fun ByteArray.gzip(): ByteArray = ByteArrayOutputStream().use { byteStream ->
    GZIPOutputStream(byteStream).use { it.write(this, 0, this.size) }
    byteStream.toByteArray()
}
