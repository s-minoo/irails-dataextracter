# IRails data extraction utilities

This repo contains two utility tools to help with extraction of iRails' logfiles:

- `./download_irails.sh [year]`: A bash script to download log files from
  [GTFS IRails site](https://gtfs.irail.be/logs/) for a particular `year`.

- Rust CLI app to process and extract log files according to specified
  iRails' `querytype`.



# Compilation of Rust CLI app

Build an executable for data extraction CLI app
```bash
cargo build -release
```

The built executable is located as: `./target/release/datacleaner-rs`


# CLI Usage

```term
Usage: datacleaner-rs [OPTIONS] <COMMAND>

Commands:
  file    process a single iRails log file
  folder  process all iRails log files under the given folder
  stdin   process all input from stdin
  help    Print this message or the help of the given subcommand(s)

Options:
  -f, --filter <QUERY_TYPES>                       Log's query types to be filtered and processed
  -d, --debug...                                   Turns on debugging and logging to file
  -o, --outputFolderSuffix <OUTPUT_FOLDER_SUFFIX>  The output folder suffix
  -h, --help                                       Print help
  -V, --version                                    Print version
```






