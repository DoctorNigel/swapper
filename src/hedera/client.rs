use std::str::FromStr;
use hedera::{
  Client, AccountId, PrivateKey
};
use crate::settings::SETTINGS;


pub fn init_hedera_client() -> Client {
  let client = Client::for_name(SETTINGS.environment.as_str()).expect("False environment variable for Hedera Client");
  let operator_id = AccountId::from_str(SETTINGS.operator_id.as_str()).expect("False operator id variable for Hedera Client");
  let private_key = PrivateKey::from_str(SETTINGS.operator_key.as_str()).expect("False private key variable for Hedera Client");
  client.set_operator(operator_id, private_key.clone());

  client
}

