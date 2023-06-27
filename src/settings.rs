use std::convert::Infallible;
use dotenvy;
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("FAILED_TO_INITIALIZE_SETTINGS");
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub environment: String,
    pub operator_id: String,
    pub operator_key: String,
    pub swap_enabled: bool
}

impl Settings {
    pub fn new() -> Result<Self, Infallible> {
        let swap_enabled = dotenvy::var("SWAP_ENABLED").expect("Swap enabled env not set").to_lowercase();
        Settings {
            environment: dotenvy::var("ENVIRONMENT").expect("Environment env error"),
            operator_id: dotenvy::var("OPERATOR_ID").expect("Operator id env error"),
            operator_key: dotenvy::var("OPERATOR_KEY").expect("Operator key env error"),
            swap_enabled: match swap_enabled.as_str() {
                "true" => true,
                _ => false
            }
        }.try_into()
    }
}