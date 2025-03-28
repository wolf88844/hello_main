use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtlpTarget {
    pub address: String,
    pub authorization: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[allow(unused)]
pub struct Database {
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[allow(unused)]
pub struct Logging {
    pub log_level: Option<String>,
    pub otlp_target: Option<OtlpTarget>,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[allow(unused)]
pub struct ConfigInfo {
    pub location: Option<String>,
    pub env_prefix: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[allow(unused)]
pub struct Settings {
    #[serde(default)]
    pub config_info: ConfigInfo,
    #[serde(default)]
    pub database: Database,
    #[serde(default)]
    pub logging: Logging,
    pub token_secret: Option<String>,
    pub token_timeout_seconds: Option<i64>,
}

impl Settings {
    pub fn new(location: Option<&str>, env_prefix: &str) -> anyhow::Result<Self> {
        let mut builder = Config::builder();
        if let Some(location) = location {
            builder = builder.add_source(File::with_name(location));
        }
        let s = builder
            .add_source(
                Environment::with_prefix(env_prefix)
                    .separator("__")
                    .prefix_separator("__"),
            )
            .set_override("config.location", location)?
            .set_override("config.env_prefix", env_prefix)?
            .build()?;

        let settings = s.try_deserialize()?;
        Ok(settings)
    }
}
