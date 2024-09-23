use std::fs::{self, File, FileType};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use category_writer::CategoryWriter;
use cli::Cli;
use error::CleanResult;
use rayon::prelude::*;
use utils::flatten_json;
use walkdir::WalkDir;

mod category_writer;
mod cli;
mod data_type;
mod error;
mod utils;

fn main() -> CleanResult<()> {
    let cli = Cli::new();
    let args = cli.parse_args();
    if let Some(file) = args.file {
        let path = PathBuf::from_str(&file).unwrap();
        let mut writer = CategoryWriter::new(path.parent().unwrap());
        process_file(&path, &mut writer)?;
    } else if let Some(folder) = args.folder {
        process_folder(folder)?;
    } else if args.is_stdin {
        unimplemented!()
    }

    Ok(())
}

fn process_folder(folder: String) -> CleanResult<()> {
    let walker = WalkDir::new(folder);

    let mut writer = CategoryWriter::default();
    for entry in walker.into_iter().filter_map(|entry| entry.ok()) {
        if entry.file_type().is_dir() {
            let mut output_dir = entry.into_path(); 
            output_dir.push("output/");
            fs::create_dir_all(&output_dir)?; 
            writer = CategoryWriter::new(&output_dir);
        } else if entry.file_type().is_file() {
            process_file(entry.path(), &mut writer)?;
        }
    }

    Ok(())
}

fn process_file(
    file_path: &Path,
    category_writer: &mut CategoryWriter,
) -> CleanResult<()> {
    let file_handle = File::open(file_path)?;
    let reader = BufReader::new(file_handle);
    let mut buffered_lines = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        buffered_lines.push(line);
        if buffered_lines.len() == 500 {
            write_out_buffered_lines(&buffered_lines, category_writer)?;

            buffered_lines.clear();
        }
    }
    if !buffered_lines.is_empty() {
        write_out_buffered_lines(&buffered_lines, category_writer)?;
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
