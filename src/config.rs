use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub port: u16,
    pub status: u16,
    pub body: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            status: 503,
            body: None,
        }
    }
}

pub async fn load_config(path: &Path) -> Config {
    let config = match std::fs::read_to_string(path) {
        Ok(config) => config,
        Err(err) => {
            tracing::warn!("Failed to read config file at {path:?}: {err}");
            return Config::default();
        }
    };

    toml::from_str(&config).unwrap_or_else(|err| {
        tracing::warn!("Failed to parse config file at {path:?}: {err}");
        Config::default()
    })
}
