mod error;
mod handlers;
mod logging;
mod models;
mod routes;
mod settings;
mod statics;

use std::sync::Arc;

use settings::Settings;
use toolcraft_axum_kit::http_server;
use toolcraft_jwt::Jwt;

use crate::{logging::init_tracing_to_file, statics::db_manager::init_db};

#[tokio::main]
async fn main() {
    init_tracing_to_file();
    let settings = Settings::load("config/services.toml").unwrap();
    init_db(settings.db.clone())
        .await
        .expect("DatabaseManager initialization failed");

    let jwt = Arc::new(Jwt::new(settings.jwt));
    let router = routes::create_routes(jwt);
    let http_task = http_server::start(settings.http.port, router);

    let _ = tokio::join!(http_task);
}
