use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use category_writer::CategoryWriter;
use cli::Cli;
use error::CleanResult;
use rayon::prelude::*;
use utils::flatten_json;

mod category_writer;
mod cli;
mod data_type;
mod error;
mod utils;

fn main() -> CleanResult<()> {
    let cli = Cli::new();
    let args = cli.parse_args();
    if let Some(file) = args.file {
        process_file(file)?;
    }

    Ok(())
}

fn process_file(file: String) -> CleanResult<()> {
    let file_path = PathBuf::from_str(&file).unwrap();
    let file_handle = File::open(file)?;
    let reader = BufReader::new(file_handle);
    let mut buffered_lines = Vec::new();
    let mut category_writer =
        CategoryWriter::new(file_path.file_stem().unwrap().to_str().unwrap());
    for line in reader.lines().map_while(Result::ok) {
        buffered_lines.push(line);
        if buffered_lines.len() == 500 {
            write_out_buffered_lines(&buffered_lines, &mut category_writer)?;

            buffered_lines.clear();
        }
    }
    if !buffered_lines.is_empty() {
        write_out_buffered_lines(&buffered_lines, &mut category_writer)?;
    }

    buffered_lines.clear();
    category_writer.flush()
}

fn write_out_buffered_lines(
    buffered_lines: &Vec<String>,
    category_writer: &mut CategoryWriter,
) -> Result<(), error::ParseError> {
    let records = buffered_lines
        .par_iter()
        .filter_map(|nldjson_line| flatten_json(nldjson_line).ok());
    category_writer.process_records(records)?;
    Ok(())
}
