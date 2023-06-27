use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use crate::routes::swap::create_swap_route;

pub(crate) mod swap;

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

pub fn create_app() -> Router {
    Router::new()
        .merge(Router::new().route("/", get(|| async { "Hi" })))
        .nest(
            "/api",
            create_swap_route()
        )
        .fallback(|| {handler_404}())
}