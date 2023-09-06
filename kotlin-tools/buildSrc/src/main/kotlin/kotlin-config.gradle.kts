import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm")
    id("com.adarshr.test-logger")
    id("org.jlleitschuh.gradle.ktlint")
}

group = rootProject.group
version = rootProject.version

repositories {
    mavenCentral()
}

kotlin {
    jvmToolchain(17)
}

tasks.test {
    useJUnitPlatform()
    outputs.upToDateWhen { false }
}
