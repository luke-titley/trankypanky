//------------------------------------------------------------------------------
// Copywrite Luke Titley 2021
//------------------------------------------------------------------------------
use super::result::Result;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

pub type ClientId = u16;
pub type TransactionId = u64;
pub type Amount = f32;

#[derive(Debug)]
pub enum Transaction {
    Chargeback {
        transaction: TransactionId,
    },
    Deposit {
        transaction: TransactionId,
        amount: Amount,
    },
    Dispute {
        transaction: TransactionId,
    },
    Resolve {
        transaction: TransactionId,
    },
    Withdrawl {
        transaction: TransactionId,
        amount: Amount,
    },
}

//------------------------------------------------------------------------------
impl std::convert::TryFrom<super::reader::IOTransaction> for Transaction {
    type Error = super::result::Error;

    fn try_from(from: super::reader::IOTransaction) -> Result<Self> {
        match from {
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Chargeback,
                client,
                tx: transaction,
                ..
            } => Ok(Self::Chargeback { transaction }),
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Deposit,
                client,
                tx: transaction,
                amount: Some(amount),
            } => Ok(Self::Deposit {
                transaction,
                amount,
            }),
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Dispute,
                client,
                tx: transaction,
                ..
            } => Ok(Self::Dispute { transaction }),
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Resolve,
                client,
                tx: transaction,
                ..
            } => Ok(Self::Resolve { transaction }),
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Withdrawl,
                client,
                tx: transaction,
                amount: Some(amount),
            } => Ok(Self::Withdrawl {
                transaction,
                amount,
            }),
            _ => Err(super::result::Error::CannotConvertFromIOTransaction),
        }
    }
}

//------------------------------------------------------------------------------
#[derive(Debug, Serialize)]
pub struct Client {
    client: u16,
    amount: Amount,
    held: Amount,
    total: Amount,
    locked: bool,

    #[serde(skip)]
    withdrawls: HashMap<u64, Amount>,
}

//------------------------------------------------------------------------------
impl Client {
    pub fn new(id: u16) -> Self {
        Self {
            client: id,
            amount: 0_f32,
            held: 0_f32,
            total: 0_f32,
            locked: false,
            withdrawls: HashMap::new(),
        }
    }

    pub fn deposit(&mut self, amount: f32) -> Result<()> {
        self.amount += amount;
        self.total += amount;

        Ok(())
    }

    pub fn withdraw(&mut self, transaction: u64, amount: f32) -> Result<()> {
        if amount > self.amount {
            return Err(super::result::Error::InsuffientFunds {
                requested: amount,
                current_balance: self.amount,
            });
        }

        self.amount -= amount;
        self.total -= amount;

        self.withdrawls.insert(transaction, amount);

        Ok(())
    }
}

pub type Clients = HashMap<u16, Client>;
