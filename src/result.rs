//------------------------------------------------------------------------------
// Copywrite Luke Titley 2021
//------------------------------------------------------------------------------
use failure::Fail;
use serde::{Deserialize, Serialize};

//------------------------------------------------------------------------------
// Error
//------------------------------------------------------------------------------
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(
        display = "invalid arguments, this takes a single csv file as its argument"
    )]
    InvalidArguments,

    #[fail(display = "given file does not exist {:?}", filepath)]
    NonExistantFile { filepath: std::path::PathBuf },

    #[fail(display = "IO error")]
    IOError(std::io::Error),

    #[fail(display = "csv error")]
    CSVError(csv::Error),

    #[fail(display = "IOTransaction is malformed")]
    CannotConvertFromIOTransaction,

    #[fail(
        display = "Insufficient funds in account. requested {}, current balance {}",
        requested, current_balance
    )]
    InsufficientFunds {
        requested: f32,
        current_balance: f32,
    },
    #[fail(display = "Transaction Id Already In Use {}", transaction)]
    TransactionIdAlreadyInUse { transaction: u64 },
}

//------------------------------------------------------------------------------
impl std::convert::From<std::io::Error> for Error {
    fn from(from: std::io::Error) -> Self {
        Error::IOError(from)
    }
}

//------------------------------------------------------------------------------
impl std::convert::From<csv::Error> for Error {
    fn from(from: csv::Error) -> Self {
        Error::CSVError(from)
    }
}

//------------------------------------------------------------------------------
pub type Result<T> = std::result::Result<T, Error>;
