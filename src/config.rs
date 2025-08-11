#[derive(Debug, schematic::Schematic, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct GcloudPluginConfig {
    pub dist_url: String,
}

impl Default for GcloudPluginConfig {
    fn default() -> Self {
        Self {
            dist_url: "https://storage.googleapis.com/storage/v1/b/cloud-sdk-release/o/google-cloud-cli-{version}-{platform}-{arch}.{ext}?alt=media".into(),
        }
    }
}
