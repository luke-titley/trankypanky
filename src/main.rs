use std::io;
use std::process;

use serde::Deserialize;
use std::path::PathBuf;

use failure::{Error, Fail};

#[derive(Debug, Deserialize)]
struct Transaction {
    type_ : String,
    client: u32,
    tx: u16,
    amount: f32,
}

#[derive(Debug, Fail)]
enum TpError {
    #[fail(display = "invalid arguments, this take a single csv file as its argument")]
    InvalidArguments,
    #[fail(display = "given file does not exist {:?}", filepath)]
    NonExistantFile {
        filepath: std::path::PathBuf,
    },
}

fn parse_arguments() -> Result<PathBuf, TpError>
{
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(TpError::InvalidArguments{});
    }

    let filepath = std::path::PathBuf::from(args[1].clone());
    if !filepath.exists() {
        return Err(TpError::NonExistantFile{filepath});
    }

    Ok(filepath)
}

fn main() -> Result<(), Error> {
    let filepath = parse_arguments()?;

    Ok(())
}

