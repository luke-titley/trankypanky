//------------------------------------------------------------------------------
// Copywrite Luke Titley 2021
//------------------------------------------------------------------------------
//! The model contains all the types pertinant to the core data model of the
//! application.
//------------------------------------------------------------------------------
use super::result::{Error, Result};
//------------------------------------------------------------------------------
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use serde::Serialize;

//------------------------------------------------------------------------------
pub type ClientId = u16;
pub type TransactionId = u64;
pub type Amount = f32;

/// Collection of clients. Indexed by the client id.
pub type Clients = HashMap<u16, Client>;

//------------------------------------------------------------------------------
#[derive(Debug)]
/// Sum type for all the operations that can be applied to a client account.
pub enum Transaction {
    /// Disputed transaction can continue, release the held money.
    Chargeback {
        transaction: TransactionId,
    },
    /// Add money to the account
    Deposit {
        transaction: TransactionId,
        amount: Amount,
    },
    /// Contest a withdrawl transaction.
    Dispute {
        transaction: TransactionId,
    },
    /// Side with the dispute, amount from the withdrawl transaction is
    /// refunded.
    Resolve {
        transaction: TransactionId,
    },
    /// Withdrawn money from the client account
    Withdrawl {
        transaction: TransactionId,
        amount: Amount,
    },
}

//------------------------------------------------------------------------------
/// Transform IOTransaction type into regular transaction.
/// Additional validation of the IOTransaction is performed here, such as
/// bounds checks around the amount.
/// This can fail if the IOTransaction is malformed.
impl std::convert::TryFrom<super::reader::IOTransaction> for Transaction {
    type Error = super::result::Error;

    fn try_from(from: super::reader::IOTransaction) -> Result<Self> {
        match from {
            // Chargeback
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Chargeback,
                tx: transaction,
                ..
            } => Ok(Self::Chargeback { transaction }),
            // Deposit
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Deposit,
                tx: transaction,
                amount: Some(amount),
                ..
            } if amount >= 0.0 => Ok(Self::Deposit {
                transaction,
                amount,
            }),
            // Dispute
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Dispute,
                tx: transaction,
                ..
            } => Ok(Self::Dispute { transaction }),
            // Resolve
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Resolve,
                tx: transaction,
                ..
            } => Ok(Self::Resolve { transaction }),
            // Withdrawl
            super::reader::IOTransaction {
                type_: super::reader::IOTransactionType::Withdrawl,
                tx: transaction,
                amount: Some(amount),
                ..
            } if amount >= 0.0 => Ok(Self::Withdrawl {
                transaction,
                amount,
            }),
            _ => Err(super::result::Error::CannotConvertFromIOTransaction),
        }
    }
}

//------------------------------------------------------------------------------
#[derive(Debug, Serialize)]
/// This manages the client account
pub struct Client {
    /// The client identifier
    client: u16,
    /// The current accessible balance
    available: Amount,
    /// The balance blocked due to dispute
    held: Amount,
    /// The total amount of money in the client account
    total: Amount,
    /// Whether the client account is blocked due to a chargeback
    locked: bool,

    #[serde(skip)]
    transactions_withdrawls: HashMap<u64, Amount>,
    #[serde(skip)]
    transactions_held: HashMap<u64, Amount>,
}

//------------------------------------------------------------------------------
impl Client {
    /// Instaniate a new Client.
    /// Must provide a client id.
    ///
    /// \param id: A client identifier, it must be unique for this client.
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

    /// Update the internal fields of the client.
    /// On concrete terms this is just updating the total balance as a sum
    /// of the held and available balances.
    ///
    /// In the future this could be used to synchronise other states.
    pub fn synchronize(&mut self) {
        self.total = self.held + self.available;
    }

    /// Increase the avilable balance in the account.
    ///
    /// \param amount: The amount to increate by.
    pub fn deposit(&mut self, amount: f32) -> Result<()> {
        assert!(amount > 0.0);
        self.available += amount;

        Ok(())
    }

    /// Decrease the available balance.
    ///
    /// \param transaction: Transaction identifier
    /// \param amount: How much to take.
    pub fn withdraw(&mut self, transaction: u64, amount: f32) -> Result<()> {
        assert!(amount > 0.0);

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

    /// Contest a previous transaction.
    ///
    /// \param transaction: The transaction identifier
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

    /// Override the withdrawl dispute. Release the held funds and lock
    /// the client account.
    ///
    /// \param transaction: The transaction identifier
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

    /// Settle a disputed transaction. When a dispute is settled the disputed
    /// funds are released, bringing the total down.
    ///
    /// \param transaction: The transaction identifier.
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

//------------------------------------------------------------------------------
/// Given a collection of clients, handle a transaction for a given client.
///
/// \param clients: All the client state
/// \param client_id: The id of the client account for the given transaction.
/// \param transaction: The operation to apply to the client account.
pub fn process_transaction(
    clients: &mut Clients,
    client_id: ClientId,
    transaction: Transaction,
) -> Result<()> {
    let client = clients.entry(client_id).or_insert(Client::new(client_id));

    match &transaction {
        // Chargeback
        Transaction::Chargeback { transaction } => {
            client.chargeback(*transaction)?;
        }

        // Make a deposit
        Transaction::Deposit { amount, .. } => {
            client.deposit(*amount)?;
        }

        // Dispute
        Transaction::Dispute { transaction } => {
            client.dispute(*transaction)?;
        }

        // Resolve
        Transaction::Resolve { transaction } => {
            client.resolve(*transaction)?;
        }

        // Withdrawl
        Transaction::Withdrawl {
            amount,
            transaction,
        } => {
            match client.withdraw(*transaction, *amount) {
                // Skip insufficient funds
                // Should probably warn, but skipping for now.
                Err(Error::InsufficientFunds { .. }) => (),

                // Skip invalid transaction
                // Should probably warn, but skipping for now.
                Err(Error::InvalidTransactionId { .. }) => (),

                // Propagate everything else
                Err(error) => return Err(error),
                _ => (),
            }
        }
    }

    Ok(())
}
