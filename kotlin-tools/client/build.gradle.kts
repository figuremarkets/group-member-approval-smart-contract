dependencies {
    listOf(
        libs.bundles.grpc,
        libs.bundles.jackson,
        libs.bundles.protobuf,
        libs.bundles.provenance,
        libs.bouncycastle,
    ).forEach(::implementation)
}
