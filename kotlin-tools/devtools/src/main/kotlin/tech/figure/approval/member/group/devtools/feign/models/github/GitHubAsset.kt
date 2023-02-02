package tech.figure.approval.member.group.devtools.feign.models.github

import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import java.time.OffsetDateTime

@JsonNaming(SnakeCaseStrategy::class)
data class GitHubAsset(
    val url: String,
    val id: Long,
    val nodeId: String,
    val name: String,
    val label: String,
    val uploader: GitHubAuthor,
    val contentType: String,
    val state: String,
    val size: Long,
    val downloadCount: Long,
    val createdAt: OffsetDateTime,
    val updatedAt: OffsetDateTime,
    val browserDownloadUrl: String,
)
