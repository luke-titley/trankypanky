//use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;

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
}

fn parse_arguments() -> Result<String, TpError>
{
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(TpError::InvalidArguments{});
    }

    let filepath = args[1].clone();

    Ok(filepath)
}

fn main() -> Result<(), Error> {
    let filepath = parse_arguments()?;

    Ok(())
}

