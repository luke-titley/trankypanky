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
fn process_transaction(
    clients: &mut model::Clients,
    client_id: model::ClientId,
    transaction: model::Transaction,
) -> Result<()> {
    println!("{:?}", transaction);

    let client = clients
        .entry(client_id)
        .or_insert(model::Client::new(client_id));

    match transaction {
        model::Transaction::Deposit { amount } => {}
        _ => (),
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
fn main() -> std::result::Result<(), failure::Error> {
    // Clients
    let mut clients = model::Clients::new();

    // Process th transactions
    let filepath = parse_arguments()?;
    reader::process_file(&filepath, |client, transaction| {
        process_transaction(&mut clients, client, transaction)
    })?;

    // Dump the results
    write(&clients);

    Ok(())
}
