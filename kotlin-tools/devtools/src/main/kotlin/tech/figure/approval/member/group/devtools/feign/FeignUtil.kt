package tech.figure.approval.member.group.devtools.feign

import com.fasterxml.jackson.databind.ObjectMapper
import feign.Feign
import feign.Logger
import feign.Request
import feign.Response
import feign.RetryableException
import feign.Retryer
import feign.codec.ErrorDecoder
import feign.jackson.JacksonDecoder
import feign.jackson.JacksonEncoder
import tech.figure.approval.member.group.client.util.GroupMemberApprovalOMUtil
import java.util.concurrent.TimeUnit

object FeignUtil {
    private val OBJECT_MAPPER by lazy { GroupMemberApprovalOMUtil.getObjectMapper() }
    private const val RETRY_MS: Long = 120000L
    private const val CONNECTION_TIMEOUT_SECONDS: Long = 1L
    private const val READ_TIMEOUT_SECONDS: Long = 60L

    fun getBuilder(mapper: ObjectMapper = OBJECT_MAPPER): Feign.Builder = Feign.builder()
        .options(
            Request.Options(
                // Connection timeout
                CONNECTION_TIMEOUT_SECONDS,
                // Connection timeout unit
                TimeUnit.SECONDS,
                // Read timeout
                READ_TIMEOUT_SECONDS,
                // Read timeout unit
                TimeUnit.SECONDS,
                // Follow redirects
                true,
            )
        )
        .logLevel(Logger.Level.BASIC)
        .logger(FeignAppLogger())
        .retryer(Retryer.Default(500, RETRY_MS, 10))
        .decode404()
        .encoder(JacksonEncoder(mapper))
        .decoder(JacksonDecoder(mapper))
        .errorDecoder(FeignErrorDecoder())

    private class FeignAppLogger : Logger() {
        override fun log(configKey: String?, format: String?, vararg args: Any?) {
            println(String.format(methodTag(configKey) + format, *args))
        }
    }

    private class FeignErrorDecoder : ErrorDecoder.Default() {
        override fun decode(methodKey: String, response: Response?): Exception = when (response?.status()) {
            502 -> retryableException(response, "502: Bad Gateway")
            503 -> retryableException(response, "503: Service Unavailable")
            504 -> retryableException(response, "504: Gateway Timeout")
            else -> super.decode(methodKey, response)
        }

        private fun retryableException(response: Response, message: String): RetryableException = RetryableException(
            // Status code
            response.status(),
            // Exception message
            message,
            // Request HTTP Method
            response.request().httpMethod(),
            // Retry after Date (set to null to indicate no retries)
            null,
            // Source request
            response.request(),
        )
    }
}
