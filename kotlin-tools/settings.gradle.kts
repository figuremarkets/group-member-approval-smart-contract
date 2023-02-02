rootProject.name = "group-member-approval-kotlin-tools"

// Allows projects to be accessed similarly to libs in dependency declarations
enableFeaturePreview("TYPESAFE_PROJECT_ACCESSORS")

include("client", "devtools")

pluginManagement {
    repositories {
        gradlePluginPortal()
    }
}

gradle.rootProject {
    allprojects {
        group = "tech.figure.approval.member.group"
        description = "Tools for interacting with the Group Member Approval Smart Contract"
    }
}

plugins {
    id("org.danilopianini.gradle-pre-commit-git-hooks") version "1.1.0"
}

gitHooks {
    preCommit {
        from {
            """
                echo "Running pre-commit ktlint check"
                # If the gradle file exists, then the terminal is currently in this repo's kotlin-tools directory
                if [ -e gradlew ]
                then
                    ./gradlew ktlintCheck
                else
                    # Else, assume the current directory is the main directory to avoid blowing up on commits
                    ./kotlin-tools/gradlew --project-dir ./kotlin-tools ktlintCheck
                fi
            """.trimIndent()
        }
    }
    createHooks()
}
