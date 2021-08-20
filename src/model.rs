//------------------------------------------------------------------------------
// Copywrite Luke Titley 2021
//------------------------------------------------------------------------------
use super::result::Result;

use serde::{Deserialize, Serialize};

use std::collections::hash_map::Entry::Vacant;
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

    pub fn synchronize(&mut self) {
        self.total = self.held + self.amount;
    }

    pub fn deposit(&mut self, amount: f32) -> Result<()> {
        self.amount += amount;

        Ok(())
    }

    pub fn withdraw(&mut self, transaction: u64, amount: f32) -> Result<()> {
        if amount > self.amount {
            return Err(super::result::Error::InsufficientFunds {
                requested: amount,
                current_balance: self.amount,
            });
        }

        match self.withdrawls.entry(transaction) {
            // Insert the new amount
            Vacant(entry) => {
                entry.insert(amount);
            }
            // If we've already used this transaction id then fail.
            _ => {
                return Err(super::result::Error::TransactionIdAlreadyInUse {
                    transaction,
                });
            }
        }

        self.amount -= amount;

        Ok(())
    }

    pub fn dispute(&mut self, transaction: u64) -> Result<()> {
        if let Some(withdrawl) = self.withdrawls.get(&transaction) {
            self.held += withdrawl;
            Ok(())
        } else {
            Err(super::result::Error::InvalidTransactionId { transaction })
        }
    }

    /*
    pub fn deposit(&mut self, amount: f32) -> Result<()> {
        self.amount += amount;
        self.total += amount;

        Ok(())
    }
    */
}

pub type Clients = HashMap<u16, Client>;
