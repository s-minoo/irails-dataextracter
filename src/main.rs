use std::collections::HashSet;
use std::fs::{self, File, FileType};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use category_writer::CategoryWriter;
use cli::Cli;
use error::CleanResult;
use log::debug;
use rayon::prelude::*;
use utils::flatten_json;
use walkdir::WalkDir;

mod category_writer;
mod cli;
mod data_type;
mod error;
mod logger;
mod utils;

fn main() -> CleanResult<()> {
    let cli = Cli::new();
    let args = cli.parse_args();
    logger::init_logger(args.is_debug).unwrap();
    if let Some(file) = args.file {
        let path = PathBuf::from_str(&file).unwrap();
        let mut writer = CategoryWriter::new(path.parent().unwrap());
        process_file(&path, &mut writer, &args.filter_query)?;
    } else if let Some(folder) = args.folder {
        process_folder(folder, &args.filter_query)?;
    } else if args.is_stdin {
        unimplemented!()
    }

    Ok(())
}

fn process_folder(
    folder: String,
    filter_query: &HashSet<String>,
) -> CleanResult<()> {
    let walker = WalkDir::new(folder);

    let mut writer = CategoryWriter::default();
    for entry in walker.into_iter().filter_map(|entry| entry.ok()) {
        if entry.file_type().is_dir()
            && entry.path().file_stem().unwrap().to_string_lossy() != "output"
        {
            let mut output_dir = entry.into_path();
            output_dir.push("output/");
            debug!("Creating output folder: {}", output_dir.to_string_lossy());
            fs::create_dir_all(&output_dir)?;

            writer = CategoryWriter::new(&output_dir);
        } else if entry.file_type().is_file()
            && entry.path().extension().unwrap() == "log"
        {
            process_file(entry.path(), &mut writer, filter_query)?;
        }
    }

    Ok(())
}

fn process_file(
    file_path: &Path,
    category_writer: &mut CategoryWriter,
    filter_query: &HashSet<String>,
) -> CleanResult<()> {
    let file_handle = File::open(file_path)?;
    let reader = BufReader::new(file_handle);
    let mut buffered_lines = Vec::new();
    debug!("Handling file: {}", file_path.to_string_lossy());
    for line in reader.lines().map_while(Result::ok) {
        buffered_lines.push(line);
        if buffered_lines.len() == 500 {
            write_out_buffered_lines(
                &buffered_lines,
                category_writer,
                filter_query,
            )?;

            buffered_lines.clear();
        }
    }
    if !buffered_lines.is_empty() {
        write_out_buffered_lines(
            &buffered_lines,
            category_writer,
            filter_query,
        )?;
    }
    buffered_lines.clear();
    debug!("Finished processing file: {}", file_path.to_string_lossy());
    category_writer.flush()
}

fn write_out_buffered_lines(
    buffered_lines: &Vec<String>,
    category_writer: &mut CategoryWriter,
    filter_query: &HashSet<String>,
) -> Result<(), error::ParseError> {
    let records = buffered_lines.par_iter().filter_map(|nldjson_line| {
        flatten_json(nldjson_line, filter_query).ok()
    });
    category_writer.process_records(records)?;
    Ok(())
}
