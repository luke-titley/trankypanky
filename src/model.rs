//------------------------------------------------------------------------------
// Copywrite Luke Titley 2021
//------------------------------------------------------------------------------
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

pub type ClientId = u16;
pub type TransactionId = u64;
pub type Amount = f32;

#[derive(Debug)]
pub enum Transaction {
    Chargeback { transaction: TransactionId },
    Deposit { amount: Amount },
    Dispute { transaction: TransactionId },
    Resolve { transaction: TransactionId },
    Withdrawl { amount: Amount },
}

//------------------------------------------------------------------------------
impl std::convert::From<super::reader::IOTransaction> for Transaction {
    fn from(from: super::reader::IOTransaction) -> Self {
        match from {
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Deposit,
                client,
                tx: transaction,
                amount: Some(amount),
            } => Self::Deposit { amount },
            _ => panic!("Only here while we fill this out"),
        }
    }
}

//------------------------------------------------------------------------------
#[derive(Debug, Serialize)]
pub struct Client {
    amount: Amount,
    held: Amount,
    total: Amount,
    locked: bool,

    #[serde(skip)]
    transactions: HashMap<u64, Transaction>,
}

pub type Clients = HashMap<u16, Client>;
