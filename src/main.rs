mod routes;
mod hedera;
mod settings;
mod utils;

use std::sync::Arc;
use ::hedera::Client;
use axum::{Extension};
use crate::hedera::client::init_hedera_client;
use crate::routes::{create_app};
use crate::settings::SETTINGS;

pub struct AppState {
    pub swap_enabled: bool,
    pub hedera_client: Client
}

#[tokio::main]
async fn main() {

    let hedera_client = init_hedera_client();
    let shared_state = Arc::new(AppState { swap_enabled: SETTINGS.swap_enabled, hedera_client });

    let app = create_app().layer(Extension(shared_state));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}