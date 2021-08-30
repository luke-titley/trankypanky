//------------------------------------------------------------------------------
// Copywrite Luke Titley 2021
//------------------------------------------------------------------------------
//! Tranky Panky is a toy account transaction summary application.
//! To build and run tranky panky you can do the following.
//!
//!
//! ```
//! cargo run -- example/simple.csv
//! ```
//!
mod model;
mod reader;
mod result;

use result::Result;

//------------------------------------------------------------------------------
/// Handle the command line arguments.
fn parse_arguments() -> Result<std::path::PathBuf> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(result::Error::InvalidArguments {});
    }

    let filepath = std::path::PathBuf::from(args[1].clone());
    if !filepath.exists() {
        return Err(result::Error::NonExistantFile { filepath });
    }

    Ok(filepath)
}

//------------------------------------------------------------------------------
/// Synchronise the internal state for all clients.
fn synchronize(clients: &mut model::Clients) -> Result<()> {
    for (_, client) in clients {
        client.synchronize();
    }

    Ok(())
}

//------------------------------------------------------------------------------
/// Display the result of the client account summary
fn write(clients: &model::Clients) -> Result<()> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    for (_, client) in clients {
        wtr.serialize(client)?;
    }
    wtr.flush()?;

    Ok(())
}

//------------------------------------------------------------------------------
/// Result handler. Resonsible for displaying all propagated errors.
fn handle<F>(f: F)
where
    F: FnOnce() -> std::result::Result<(), failure::Error>,
{
    let result = f();
    match result {
        Err(error) => panic!("{}", error),
        Ok(()) => (),
    }
}

//------------------------------------------------------------------------------
/// you know what this does.
fn main() {
    // Panic with any errors and format them correctly to stderr
    handle(|| {
        // Clients
        let mut clients = model::Clients::new();

        // Process th transactions
        let filepath = parse_arguments()?;
        reader::process_file(&filepath, |client, transaction| {
            model::process_transaction(&mut clients, client, transaction)
        })?;

        // Synchronize clients
        synchronize(&mut clients)?;

        // Dump the results
        write(&clients)?;

        Ok(())
    })
}
