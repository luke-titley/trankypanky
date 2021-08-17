//------------------------------------------------------------------------------
// Copywrite Luke Titley 2021
//------------------------------------------------------------------------------
use super::model;
use super::result::Result;
//------------------------------------------------------------------------------
use serde::Deserialize;

//------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IOTransactionType {
    Chargeback,
    Deposit,
    Dispute,
    Resolve,
    Withdrawl,
}

//------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct IOTransaction {
    #[serde(rename = "type")]
    pub type_: IOTransactionType,
    pub client: u16,
    pub tx: u64,
    pub amount: Option<f32>,
}

//------------------------------------------------------------------------------
pub fn process_file<H>(filepath: &std::path::Path, mut handler: H) -> Result<()>
where
    H: FnMut(model::ClientId, model::Transaction) -> Result<()>,
{
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(filepath)?;

    for result in rdr.deserialize() {
        // Deserialize the transaction
        let transaction: IOTransaction = result?;
        handler(
            transaction.client,
            <model::Transaction as std::convert::From<
                super::reader::IOTransaction,
            >>::from(transaction),
        )?;
    }

    Ok(())
}
