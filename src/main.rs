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
    transaction: &model::Transaction,
) -> Result<()> {
    Ok(())
}

//------------------------------------------------------------------------------
fn main() -> std::result::Result<(), failure::Error> {
    let filepath = parse_arguments()?;

    reader::process_file(&filepath, |transaction| {
        println!("{:?}", transaction);

        Ok(())
    })?;

    Ok(())
}
