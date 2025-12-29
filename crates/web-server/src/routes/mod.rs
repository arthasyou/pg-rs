mod example;

use std::sync::Arc;

use axum::{Extension, Router, middleware::from_fn};
use toolcraft_axum_kit::middleware::{auth_mw::auth, cors::create_cors};
use toolcraft_jwt::Jwt;
use utoipa::{
    OpenApi,
    openapi::security::{ApiKey, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::example::ExampleApi;

#[derive(OpenApi)]
#[openapi(
        nest(
            (path = "/example", api = ExampleApi),
        ),
    )]
struct ApiDoc;

pub fn create_routes(jwt: Arc<Jwt>) -> Router {
    let cors = create_cors();
    let mut doc = ApiDoc::openapi();
    doc.components
        .get_or_insert_with(Default::default)
        .add_security_scheme(
            "Bearer",
            SecurityScheme::ApiKey(ApiKey::Header(
                utoipa::openapi::security::ApiKeyValue::with_description(
                    "Authorization",
                    "请输入格式：Bearer <token>",
                ),
            )),
        );

    Router::new()
        .nest("/example", example::example_routes())
        .route_layer(from_fn(auth))
        .layer(Extension(jwt))
        .layer(cors)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc))
}
