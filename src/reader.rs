//------------------------------------------------------------------------------
// Copywrite Luke Titley 2021
//------------------------------------------------------------------------------
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
    type_: IOTransactionType,
    client: u16,
    tx: u64,
    amount: Option<f32>,
}

//------------------------------------------------------------------------------
pub fn process_file<H>(filepath: &std::path::Path, mut handler: H) -> Result<()>
where
    H: FnMut(&IOTransaction) -> Result<()>,
{
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(filepath)?;

    for result in rdr.deserialize() {
        let transaction: IOTransaction = result?;
        handler(&transaction)?;
    }

    Ok(())
}
