import org.gradle.api.publish.maven.MavenPublication
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import java.net.URI

plugins {
    `maven-publish`
    `java-library`
    signing
    id("io.github.gradle-nexus.publish-plugin")
}

val projectGroup = rootProject.group
val projectVersion = project.property("version").takeIf { it != "unspecified" } ?: "1.0-SNAPSHOT"

configure<io.github.gradlenexus.publishplugin.NexusPublishExtension> {
    repositories {
        sonatype {
            nexusUrl.set(uri("https://s01.oss.sonatype.org/service/local/"))
            snapshotRepositoryUrl.set(uri("https://s01.oss.sonatype.org/content/repositories/snapshots/"))
            username.set(findProject("ossrhUsername")?.toString() ?: System.getenv("OSSRH_USERNAME"))
            password.set(findProject("ossrhPassword")?.toString() ?: System.getenv("OSSRH_PASSWORD"))
            stagingProfileId.set("858b6e4de4734a") // tech.figure staging profile id
        }
    }
}

subprojects {
    apply {
        plugin("maven-publish")
        plugin("kotlin")
        plugin("java-library")
        plugin("signing")
        plugin("kotlin-config")
    }

    java {
        withSourcesJar()
        withJavadocJar()
    }

    // Add a "member-approval" prefix to each library's name.  This will prevent jar collisions with other libraries
    // that have ambiguously-named resources like this one, eg: client-1.0.0.jar == bad
    val artifactName = "member-approval-$name"
    val artifactVersion = projectVersion.toString()


    tasks.withType<PublishToMavenLocal> {
        signing.isRequired = false
    }

    configure<PublishingExtension> {
        publications {
            create<MavenPublication>("maven") {
                groupId = projectGroup.toString()
                artifactId = artifactName
                version = artifactVersion

                from(components["java"])

                pom {
                    name.set("Figure Tech Group Member Approval Kotlin Library: $artifactName")
                    description.set("Various tools for interacting with the Group Member Approval Smart Contract")
                    url.set("https://figure.tech")
                    licenses {
                        license {
                            name.set("The Apache License, Version 2.0")
                            url.set("http://www.apache.org/licenses/LICENSE-2.0.txt")
                        }
                    }

                    developers {
                        developer {
                            id.set("FigureTechnologies/team-rd")
                            name.set("Figure Technologies Research and Development Team")
                        }
                    }
                    scm {
                        developerConnection.set("git@github.com:FigureTechnologies/group-member-approval-smart-contract.git")
                        connection.set("https://github.com/FigureTechnologies/group-member-approval-smart-contract.git")
                        url.set("https://github.com/FigureTechnologies/group-member-approval-smart-contract")
                    }
                }
            }
        }

        if (!System.getenv("DISABLE_SIGNING").toBoolean()) {
            configure<SigningExtension> {
                sign(publications["maven"])
            }
        }
    }
}

