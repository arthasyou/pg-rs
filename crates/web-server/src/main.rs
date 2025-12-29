mod error;
mod handlers;
mod logging;
mod models;
mod routes;
mod settings;

use std::sync::Arc;

use settings::Settings;
use toolcraft_axum_kit::http_server;
use toolcraft_jwt::Jwt;

use crate::logging::init_tracing_to_file;

#[tokio::main]
async fn main() {
    init_tracing_to_file();
    let settings = Settings::load("config/services.toml").unwrap();

    let jwt = Arc::new(Jwt::new(settings.jwt));
    let router = routes::create_routes(jwt);
    let http_task = http_server::start(settings.http.port, router);

    let _ = tokio::join!(http_task);
}
