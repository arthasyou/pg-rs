mod medical;

use std::sync::Arc;

use axum::{Extension, Router};
use toolcraft_axum_kit::middleware::cors::create_cors;
use toolcraft_jwt::Jwt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::medical::MedicalApi;

#[derive(OpenApi)]
#[openapi(
        nest(
            (path = "/medical", api = MedicalApi),
        ),
    )]
struct ApiDoc;

pub fn create_routes(jwt: Arc<Jwt>) -> Router {
    let cors = create_cors();
    let doc = ApiDoc::openapi();

    Router::new()
        .nest("/medical", medical::medical_routes())
        .layer(Extension(jwt))
        .layer(cors)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc))
}
