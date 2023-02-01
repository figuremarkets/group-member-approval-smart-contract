configurations.all {
    exclude(group = "log4j")
}

plugins {
    id("publish-kotlin-artifact")
}
