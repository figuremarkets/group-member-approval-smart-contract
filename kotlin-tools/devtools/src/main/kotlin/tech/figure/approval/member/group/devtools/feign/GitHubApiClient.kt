package tech.figure.approval.member.group.devtools.feign

import feign.Param
import feign.RequestLine
import tech.figure.approval.member.group.devtools.feign.models.github.GitHubReleaseResponse

/**
 * Simple client to communicate with GitHub's open api to retrieve an asset for an open source project.
 */
interface GitHubApiClient {
    @RequestLine("GET /repos/{organization}/{repository}/releases/latest")
    fun getLatestRelease(
        @Param("organization") organization: String,
        @Param("repository") repository: String,
    ): GitHubReleaseResponse

    @RequestLine("GET /repos/{organization}/{repository}/releases/tags/{tag}")
    fun getReleaseByTag(
        @Param("organization") organization: String,
        @Param("repository") repository: String,
        @Param("tag") tag: String,
    ): GitHubReleaseResponse

    companion object {
        fun new(): GitHubApiClient = FeignUtil.getBuilder().target(GitHubApiClient::class.java, "https://api.github.com")
    }
}
