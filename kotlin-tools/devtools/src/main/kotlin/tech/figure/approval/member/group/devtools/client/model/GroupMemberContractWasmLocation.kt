package tech.figure.approval.member.group.devtools.client.model

/**
 * Denotes a location of the smart contract for use in storage.
 */
sealed interface GroupMemberContractWasmLocation {
    /**
     * Uses the group member approval smart contract's repository to download via GitHub's exposed api.
     *
     * @param contractReleaseTag The release version used to download the contract.  If no version is specified, the
     * latest version is fetched.  If a missing version is specified, an error will occur.
     */
    class GitHub(val contractReleaseTag: String? = null) : GroupMemberContractWasmLocation

    /**
     * Denotes that the contract is in a local area on the current machine's environment.
     */
    sealed interface LocalFile : GroupMemberContractWasmLocation {
        /**
         * Use an absolute path on the current machine to find a contract instance.
         *
         * @param absoluteFilePath An absolute path to the contract on the current machine using standard unix definitions.
         */
        class AbsolutePath(val absoluteFilePath: String) : LocalFile

        /**
         * Use a project resource folder value.
         *
         * @param resourcePath The path within the resources folder.  Ex: wasms/artifacts/mywasm.wasm
         */
        class ProjectResource(val resourcePath: String) : LocalFile
    }
}
