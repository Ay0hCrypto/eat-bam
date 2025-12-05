pub fn get_client_version_config() -> super::ClientVersionConfig {
    super::ClientVersionConfig {
        client_name: "bam",
        is_submodule: false,
        fallback_version_getter: None,
    }
}
