use std::sync::Arc;
use axum::{Router, Json, Extension};
use axum::extract::{Query};
use axum::response::IntoResponse;
use axum::routing::get;
use crate::AppState;
use crate::hedera::{create_transfer_transaction};
use crate::settings::SETTINGS;
use crate::utils::custom_response::{CodeResponse, ErrorResponse, SwapRequest, SwapStatus, TransactionResponse};


pub fn create_swap_route() -> Router {
    Router::new()
        .route("/swap", get(create_swap))
        .route("/swap/status", get(swap_status))
}

pub async fn swap_status(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    match state.swap_enabled {
        true => Json(SwapStatus { swap_available: true }),
        false => Json(SwapStatus { swap_available: false })
    }
}

pub async fn create_swap(opts: Option<Query<SwapRequest>>, Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let Query(opts) = opts.unwrap_or_default();

    println!("GET request for swap route for accountId {}, swap_enabled is currently {}", opts.account_id, state.swap_enabled);
    if !state.swap_enabled { return Err(Json(ErrorResponse {
        error: CodeResponse {
            code: "Swap is currently disabled".to_owned()
            }
        }))
    }

    let transaction = create_transfer_transaction(
        opts.token_id, opts.account_id, &SETTINGS.operator_id, opts.amount,
        opts.amount, 9
    );

    let checked_trans = match transaction {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("{}", e);
            return Err(Json(ErrorResponse {
                error: CodeResponse {
                    code: "Error with generating transaction".to_owned()
                }
            }));
        }
    };

    let response = TransactionResponse::new(checked_trans);
    Ok(Json(response))
}