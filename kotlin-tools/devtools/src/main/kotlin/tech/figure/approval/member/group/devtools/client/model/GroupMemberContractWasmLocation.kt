package tech.figure.approval.member.group.devtools.client.model

sealed interface GroupMemberContractWasmLocation {
    class GitHub(val contractReleaseTag: String? = null) : GroupMemberContractWasmLocation
    sealed interface LocalFile : GroupMemberContractWasmLocation {
        class AbsolutePath(val absoluteFilePath: String) : LocalFile
        class ProjectResource(val resourcePath: String) : LocalFile
    }
}
