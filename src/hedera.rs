pub(crate) mod client;

use std::str::FromStr;
use hedera::{Hbar, AccountId, TransferTransaction, TokenId, TransactionId, Result,
};
use crate::hedera::client::{CLIENT};

pub fn create_transfer_transaction(token_id: String,
                                   payer_account: String,
                                   owner_account: &str,
                                   hbar_amount: i64,
                                   token_amount: i64,
                                   decimals: u32) -> Result<Vec<u8>> {
    let token_id = TokenId::from_str(&*token_id)?;
    let payer_id = AccountId::from_str(&*payer_account)?;
    let owner_id = AccountId::from_str(owner_account)?;

    println!("Token amount is: {}", token_amount);
    println!("{}", token_amount.pow(decimals));

    TransferTransaction::new()
        .node_account_ids(vec![AccountId::from(3)])
        .transaction_id(TransactionId::generate(payer_id))
        .token_transfer(token_id, owner_id, -token_amount.pow(decimals))
        .token_transfer(token_id, payer_id, token_amount.pow(decimals))
        .hbar_transfer(owner_id, Hbar::new(hbar_amount))
        .hbar_transfer(payer_id, Hbar::new(-hbar_amount))
        .freeze_with(&*CLIENT).unwrap().sign_with_operator(&CLIENT).unwrap()
        .to_bytes()
}

/*
pub struct PendingTransaction {
    owner: AccountId,
    payer: AccountId,
    token_id: Option<TokenId>,
    hbar_amount: i64,
    token_amount: i64,
    token_decimals: Option<u32>
}

impl PendingTransaction {
    pub fn new(token_id: String,
               payer_account: String,
               owner_account: &str,
               hbar_amount: i64,
               token_amount: i64,
               decimals: u32) -> Self {
        PendingTransaction {
            owner: AccountId::from_str(&*owner_account).unwrap(),
            payer: AccountId::from_str(&*payer_account).unwrap(),
            token_id: Option::from(TokenId::from_str(&*token_id).unwrap()),
            hbar_amount,
            token_amount,
            token_decimals: Option::from(decimals),
        }
    }
}

pub fn new_transfer_transaction(token_id: String,
                                payer_account: String,
                                owner_account: &str,
                                hbar_amount: i64,
                                token_amount: i64,
                                decimals: u32) -> PendingTransaction {
    PendingTransaction::new(token_id,
                            payer_account,
                            owner_account,
                            hbar_amount,
                            token_amount,
                            decimals)
}
 */




