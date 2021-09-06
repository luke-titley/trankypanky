//------------------------------------------------------------------------------
// Copywrite Luke Titley 2021
//------------------------------------------------------------------------------
//! For all error and result types.
use failure::Fail;

//------------------------------------------------------------------------------
// Error
//------------------------------------------------------------------------------
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(
        display = "invalid arguments, this takes a single csv file as its argument"
    )]
    /// The arguments given to this application are incorrect.
    InvalidArguments,

    #[fail(display = "given file does not exist {:?}", filepath)]
    /// The given file does not exist at this path
    NonExistantFile { filepath: std::path::PathBuf },

    #[fail(display = "IO error")]
    /// Input/Ouput errors like permission errors, etc
    IOError(std::io::Error),

    #[fail(display = "csv error")]
    /// Error parsing/reading the input csv file
    CSVError(csv::Error),

    #[fail(display = "IOTransaction is malformed")]
    /// Input is parsable but there are other errors in the file like negative
    /// deposit amounts
    CannotConvertFromIOTransaction,

    #[fail(
        display = "Insufficient funds in account. requested {}, current balance {}",
        requested, current_balance
    )]
    /// The funds requested for withdrawal are greater than the funds that are
    /// available.
    InsufficientFunds {
        requested: f32,
        current_balance: f32,
    },
    #[fail(display = "Transaction Id Already In Use {}", transaction)]

    /// A transaction id is being used, but its already been used before.
    /// This is for withdrawal transactions.
    TransactionIdAlreadyInUse { transaction: u64 },

    #[fail(display = "Invalid transaction id {}", transaction)]

    /// The given transaction id is invalid. This error happens for
    /// transactions that reference others, like disputes. If
    /// the disputed transaction references an invalid transaction id
    /// you'll get this error.
    InvalidTransactionId { transaction: u64 },
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
