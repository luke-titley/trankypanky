use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Transaction {
    type_ : String,
    client: u32,
    tx: u16,
    amount: f32,
}

fn main() {

    for argument in std::env::args() {
        println!("{}", argument);
    }

    println!("Hello, world!");
}
