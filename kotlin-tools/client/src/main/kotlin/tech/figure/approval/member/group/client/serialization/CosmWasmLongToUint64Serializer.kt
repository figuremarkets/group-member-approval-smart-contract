package tech.figure.approval.member.group.client.serialization

import com.fasterxml.jackson.core.JsonGenerator
import com.fasterxml.jackson.databind.JsonSerializer
import com.fasterxml.jackson.databind.SerializerProvider

class CosmWasmLongToUint64Serializer : JsonSerializer<Long>() {
    override fun serialize(value: Long?, gen: JsonGenerator?, serializers: SerializerProvider?) {
        value?.also { long ->
            require(long >= 0) { "CosmWasm Uint64 type requires unsigned integers" }
            gen?.writeString(long.toString())
        }
    }
}
