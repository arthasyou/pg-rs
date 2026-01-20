use pg_core::DatabaseConfig;
use serde::Deserialize;
use toolcraft_config::load_settings;
use toolcraft_jwt::JwtCfg;

use crate::error::Result;
use crate::utils::LlmConfig;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub http: HttpCfg,
    pub jwt: JwtCfg,
    pub db: Vec<DatabaseConfig>,
    #[serde(default)]
    pub llm: LlmConfig,
}

#[derive(Debug, Deserialize)]
pub struct HttpCfg {
    pub port: u16,
}

impl Settings {
    pub fn load(config_path: &str) -> Result<Self> {
        let r = load_settings(config_path)?;
        Ok(r)
    }
}
