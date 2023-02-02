dependencies {
    listOf(
        projects.client,
        libs.bundles.provenance,
        libs.feign.jackson,
    ).forEach(::implementation)
}
