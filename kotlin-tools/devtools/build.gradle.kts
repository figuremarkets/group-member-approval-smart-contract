dependencies {
    listOf(
        projects.client,
        libs.feign.jackson,
    ).forEach(::implementation)
}
