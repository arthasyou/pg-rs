use serde::Deserialize;
use toolcraft_config::load_settings;
use toolcraft_jwt::JwtCfg;

use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub http: HttpCfg,
    pub jwt: JwtCfg,
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
