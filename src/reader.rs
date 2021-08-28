//------------------------------------------------------------------------------
// Copywrite Luke Titley 2021
//------------------------------------------------------------------------------
//! All types pertaining to handling file input.

use super::model;
use super::result::Result;
//------------------------------------------------------------------------------
use serde::Deserialize;

//------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
/// Transaction type id used for IO only. Enum used to identify the different
/// types of transactions.
pub enum IOTransactionType {
    Chargeback,
    Deposit,
    Dispute,
    Resolve,
    Withdrawl,
}

//------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
/// Transaction type used for IO only. This IOTransaction will be converted into
/// a transaction that is simpler to work with in the client api.
pub struct IOTransaction {
    #[serde(rename = "type")]
    /// The type of transaction
    pub type_: IOTransactionType,
    /// The client whose account the transaction affects
    pub client: u16,
    /// The transaction id.
    pub tx: u64,
    /// For deposit and withdrawl transactions, the amount affected.
    pub amount: Option<f32>,
}

//------------------------------------------------------------------------------
/// This will iterate over all the transactions in a input file, executing the
/// handler for each transaction.
pub fn process_file<H>(filepath: &std::path::Path, mut handler: H) -> Result<()>
where
    H: FnMut(model::ClientId, model::Transaction) -> Result<()>,
{
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(filepath)?;

    // Loop over each line of the file
    for result in rdr.deserialize() {
        // Deserialize the transaction
        let io_transaction: IOTransaction = result?;
        let client = io_transaction.client;

        // Validate the transaction
        let transaction = <model::Transaction as std::convert::TryFrom<
            super::reader::IOTransaction,
        >>::try_from(io_transaction)?;

        // Run the transaction handler
        handler(client, transaction)?;
    }

    Ok(())
}
