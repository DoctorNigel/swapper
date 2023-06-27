use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Default)]
pub struct SwapRequest {
    pub account_id: String,
    pub network: Option<String>,
    pub token_id: String,
    pub amount: i64
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub bytes: Vec<u8>
}

impl TransactionResponse {
    pub fn new(bytes: Vec<u8>) -> Self {
        TransactionResponse {
            bytes
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: CodeResponse
}

#[derive(Serialize)]
pub struct CodeResponse {
    pub code: String
}

#[derive(Serialize)]
pub struct SwapStatus {
    pub swap_available: bool
}