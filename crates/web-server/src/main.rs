mod dto;
mod error;
mod handlers;
mod logging;
mod routes;
mod settings;
mod statics;
mod utils;

use std::sync::Arc;

use settings::Settings;
use toolcraft_axum_kit::http_server;
use toolcraft_jwt::Jwt;

use crate::{
    logging::init_tracing_to_file, statics::db_manager::init_db,
    statics::llm_client::init_llm,
};

#[tokio::main]
async fn main() {
    init_tracing_to_file();
    let settings = Settings::load("config/services.toml").unwrap();
    init_db(settings.db.clone())
        .await
        .expect("DatabaseManager initialization failed");

    init_llm(settings.llm).expect("LLM Client initialization failed");

    let jwt = Arc::new(Jwt::new(settings.jwt));
    let router = routes::create_routes(jwt);
    let http_task = http_server::start(settings.http.port, router);

    let _ = tokio::join!(http_task);
}
