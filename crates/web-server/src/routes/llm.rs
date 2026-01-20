// LLM 路由示例
// 可复制到 src/routes/ 中使用或集成到现有路由中

use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::llm::{generate_text, get_model_info};

pub fn create_llm_routes() -> Router {
    Router::new()
        .route("/model/info", get(get_model_info))
        .route("/generate", post(generate_text))
        .fallback(|| async { "LLM API" })
}
