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
    // TODO: sc-265993 - Remove after pb client 2.4.0 is fully released
    maven {
        url = uri("https://s01.oss.sonatype.org/content/groups/staging/")
    }
}

kotlin {
    jvmToolchain(17)
}

tasks.test {
    useJUnitPlatform()
    outputs.upToDateWhen { false }
}
