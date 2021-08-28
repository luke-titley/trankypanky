//------------------------------------------------------------------------------
// Copywrite Luke Titley 2021
//------------------------------------------------------------------------------
mod model;
mod reader;
mod result;

use result::Result;

//------------------------------------------------------------------------------
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
fn synchronize(clients: &mut model::Clients) -> Result<()> {
    for (_, client) in clients {
        client.synchronize();
    }

    Ok(())
}

//------------------------------------------------------------------------------
fn write(clients: &model::Clients) -> Result<()> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    for (_, client) in clients {
        wtr.serialize(client)?;
    }
    wtr.flush()?;

    Ok(())
}

//------------------------------------------------------------------------------
fn handle<F>(f : F)
    where
        F : FnOnce() -> std::result::Result<(), failure::Error>
{
    let result = f();
    match result {
        Err(error) => panic!("{}", error),
        Ok(()) => (),
    }
}

//------------------------------------------------------------------------------
fn main() {
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
