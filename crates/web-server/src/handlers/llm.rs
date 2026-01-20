// LLM Handler 示例
// 可复制到 src/handlers/ 中使用

use axum::Json;
use serde_json::{json, Value};

use crate::statics::llm_client::get_llm_config;

#[derive(serde::Deserialize)]
pub struct GenerateRequest {
    pub prompt: String,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub top_p: Option<f32>,
}

#[derive(serde::Serialize)]
pub struct GenerateResponse {
    pub response: String,
    pub model: String,
    pub total_duration: u64,
}

/// 文本生成端点
pub async fn generate_text(Json(req): Json<GenerateRequest>) -> Json<Value> {
    let config = get_llm_config();

    // 这里示例使用 get_llm_config() 获取配置
    // 实际实现需要调用 Ollama API
    let base_url = &config.base_url;
    let model = &config.model;

    // 示例响应
    let response = json!({
        "status": "success",
        "message": format!("Would call {} with model {}", base_url, model),
        "prompt": req.prompt,
        "base_url": base_url,
        "model": model,
    });

    Json(response)
}

/// 模型信息端点
pub async fn get_model_info() -> Json<Value> {
    let config = get_llm_config();

    let info = json!({
        "base_url": &config.base_url,
        "model": &config.model,
        "has_api_key": !config.api_key.is_empty(),
    });

    Json(info)
}
