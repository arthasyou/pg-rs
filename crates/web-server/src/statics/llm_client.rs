use std::sync::OnceLock;

use crate::utils::LlmConfig;
use crate::error::Result;

static LLM_CONFIG: OnceLock<LlmConfig> = OnceLock::new();

pub fn init_llm(config: LlmConfig) -> Result<()> {
    LLM_CONFIG
        .set(config.clone())
        .map_err(|_| crate::error::Error::Custom("LLM Config already initialized".to_owned()))?;

    let cfg = get_llm_config();
    tracing::info!(
        "LLM Config initialized - Base URL: {}, Model: {}",
        cfg.base_url,
        cfg.model,
    );

    Ok(())
}

pub fn get_llm_config() -> &'static LlmConfig {
    LLM_CONFIG.get().expect("LLM Config not initialized")
}

