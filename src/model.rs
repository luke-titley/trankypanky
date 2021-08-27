//------------------------------------------------------------------------------
// Copywrite Luke Titley 2021
//------------------------------------------------------------------------------
use super::result::Result;

use serde::Serialize;

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
                tx: transaction,
                ..
            } => Ok(Self::Chargeback { transaction }),
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Deposit,
                tx: transaction,
                amount: Some(amount),
                ..
            } => Ok(Self::Deposit {
                transaction,
                amount,
            }),
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Dispute,
                tx: transaction,
                ..
            } => Ok(Self::Dispute { transaction }),
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Resolve,
                tx: transaction,
                ..
            } => Ok(Self::Resolve { transaction }),
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Withdrawl,
                tx: transaction,
                amount: Some(amount),
                ..
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
    available: Amount,
    held: Amount,
    total: Amount,
    locked: bool,

    #[serde(skip)]
    transactions_withdrawls: HashMap<u64, Amount>,
    #[serde(skip)]
    transactions_held: HashMap<u64, Amount>,
}

//------------------------------------------------------------------------------
impl Client {
    pub fn new(id: u16) -> Self {
        Self {
            client: id,
            available: 0_f32,
            held: 0_f32,
            total: 0_f32,
            locked: false,
            transactions_withdrawls: HashMap::new(),
            transactions_held: HashMap::new(),
        }
    }

    pub fn synchronize(&mut self) {
        self.total = self.held + self.available;
    }

    pub fn deposit(&mut self, amount: f32) -> Result<()> {
        self.available += amount;

        Ok(())
    }

    pub fn withdraw(&mut self, transaction: u64, amount: f32) -> Result<()> {
        if amount > self.available {
            return Err(super::result::Error::InsufficientFunds {
                requested: amount,
                current_balance: self.available,
            });
        }

        match self.transactions_withdrawls.entry(transaction) {
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

        self.available -= amount;

        Ok(())
    }

    pub fn dispute(&mut self, transaction: u64) -> Result<()> {
        // Remove the withdrawl entry to protect against multiple disputes
        // of the same transaction.
        if let Some(amount) = self.transactions_withdrawls.remove(&transaction)
        {
            assert!(self.transactions_held.get(&transaction).is_none());

            self.held += amount;
            self.transactions_held.insert(transaction, amount);
        } else {
            // Ignore invalid transaction id (as specified by spec)
        }

        Ok(())
    }

    pub fn chargeback(&mut self, transaction: u64) -> Result<()> {
        if let Some(amount) = self.transactions_held.remove(&transaction) {
            assert!(self.held >= amount);

            self.held -= amount;
            self.locked = true;

            Ok(())
        } else {
            Err(super::result::Error::InvalidTransactionId { transaction })
        }
    }

    pub fn resolve(&mut self, transaction: u64) -> Result<()> {
        if let Some(amount) = self.transactions_held.remove(&transaction) {
            assert!(self.held >= amount);

            self.held -= amount;
            self.available += amount;

            Ok(())
        } else {
            Err(super::result::Error::InvalidTransactionId { transaction })
        }
    }
}

pub type Clients = HashMap<u16, Client>;
