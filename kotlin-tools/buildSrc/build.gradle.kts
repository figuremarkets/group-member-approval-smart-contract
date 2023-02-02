plugins {
    `kotlin-dsl`
}

repositories {
    mavenLocal()
    mavenCentral()
    gradlePluginPortal()
}

dependencies {
    implementation("org.jetbrains.kotlin", "kotlin-gradle-plugin", "1.7.22")
    implementation("io.github.gradle-nexus", "publish-plugin", "1.1.0")
    implementation("com.adarshr", "gradle-test-logger-plugin", "3.2.0")
    implementation("org.jlleitschuh.gradle", "ktlint-gradle", "11.0.0")
}
