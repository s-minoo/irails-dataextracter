use cli::Cli;
use error::ParseResult;

mod cli;
mod error;

fn main() -> ParseResult<()> {
    let cli = Cli::new();
    let args = cli.parse_args();
    if let Some(file) = args.file {
        process_file(file)?;
    }

    Ok(())
}

fn process_file(file: String) -> ParseResult<()> {
    todo!()
}
