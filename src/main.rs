mod routes;
mod hedera;
mod settings;
mod utils;

use std::sync::Arc;
use axum::{Extension};
use crate::routes::{create_app};
use crate::settings::SETTINGS;

pub struct AppState {
    pub swap_enabled: bool
}

#[tokio::main]
async fn main() {

    let shared_state = Arc::new(AppState { swap_enabled: SETTINGS.swap_enabled});

    let app = create_app().layer(Extension(shared_state));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}