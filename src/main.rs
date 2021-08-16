use std::io;
use std::process;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use failure::{Error, Fail};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Type {
    Chargeback,
    Deposit,
    Dispute,
    Resolve,
    Withdrawl,
}

type ClientId = u16;

#[derive(Debug, Deserialize)]
struct Transaction {
    #[serde(rename = "type")]
    type_: Type,
    client: ClientId,
    tx: u64,
    amount: Option<f32>,
}

#[derive(Debug, Serialize)]
struct Client {
    client: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
    #[serde(skip)]
    disputed: Vec<u64>,
}

type Clients = HashMap<u16, Client>;

#[derive(Debug, Fail)]
enum TpError {
    #[fail(display = "invalid arguments, this take a single csv file as its argument")]
    InvalidArguments,

    #[fail(display = "given file does not exist {:?}", filepath)]
    NonExistantFile { filepath: std::path::PathBuf },

    #[fail(display = "IO error")]
    IOError(std::io::Error),

    #[fail(display = "csv error")]
    CSVError(csv::Error),
}

impl std::convert::From<std::io::Error> for TpError {
    fn from(from: std::io::Error) -> Self {
        TpError::IOError(from)
    }
}

impl std::convert::From<csv::Error> for TpError {
    fn from(from: csv::Error) -> Self {
        TpError::CSVError(from)
    }
}

fn parse_arguments() -> Result<PathBuf, TpError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(TpError::InvalidArguments {});
    }

    let filepath = std::path::PathBuf::from(args[1].clone());
    if !filepath.exists() {
        return Err(TpError::NonExistantFile { filepath });
    }

    Ok(filepath)
}

fn process_file<H>(filepath: &std::path::Path, mut handler: H) -> Result<(), TpError>
where
    H: FnMut(&Transaction) -> Result<(), TpError>,
{
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(filepath)?;

    for result in rdr.deserialize() {
        let transaction: Transaction = result?;
        handler(&transaction)?;
    }

    Ok(())
}

fn process_transaction(clients: &mut Clients, transaction : &Transaction) ->Result<(), TpError> {
    Ok(())
}

fn main() -> Result<(), Error> {
    let filepath = parse_arguments()?;

    process_file(&filepath, |transaction| {
        println!("{:?}", transaction);

        Ok(())
    })?;

    Ok(())
}
