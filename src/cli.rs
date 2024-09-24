use std::collections::HashSet;

use clap::{arg, Command};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Cli {
    cmd: Command,
}

#[derive(Debug, Clone)]
pub struct ParsedArguments {
    pub file:                 Option<String>,
    pub folder:               Option<String>,
    pub is_stdin:             bool,
    pub is_debug:             bool,
    pub filter_query:         HashSet<String>,
    pub output_folder_suffix: String,
}

impl Default for ParsedArguments {
    fn default() -> Self {
        Self {
            file:                 Default::default(),
            folder:               Default::default(),
            is_stdin:             false,
            is_debug:             false,
            filter_query:         ["occupancy".to_string()].into(),
            output_folder_suffix: "generated_csvs".to_string(),
        }
    }
}

impl Cli {
    pub fn new() -> Cli {
        let cmd = Command::new("Big Data Science iRails data cleaner")
            .version(VERSION)
            .author("Sitt Min Oo")
            .about(format!("Cleans the iRails log's nldjson files into CSV categorized into different iRails query type.\n\
                Current version is {}", VERSION))
            .subcommand_required(true)
            .propagate_version(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("file")
                         .about("process a single iRails log file")
                         .arg(arg!(<DOCUMENT> "the log file to be processed"))
                         .arg_required_else_help(true))
            .subcommand(Command::new("folder")
                         .about("process all iRails log files under the given folder")
                         .arg(arg!(<FOLDER> "the folder containing several iRails log files"))
                         .arg_required_else_help(true))
            .subcommand(Command::new("stdin")
                         .about("process all input from stdin"))
            .arg(arg!(-f --filter <QUERY_TYPES> "Log's query types to be filtered and processed"))
            .arg(arg!(-d --debug ...  "Turns on debugging and logging to file"))
            .arg(arg!(-o --outputFolderSuffix <OUTPUT_FOLDER_SUFFIX> "The output folder suffix"));

        Self { cmd }
    }

    pub fn parse_args(self) -> ParsedArguments {
        let arg_matches = self.cmd.get_matches();
        let is_debug = *arg_matches.get_one::<u8>("debug").unwrap() > 0;
        let mut filter_query = arg_matches
            .get_many::<String>("filter")
            .unwrap_or_default()
            .map(|v| v.to_string())
            .collect::<HashSet<_>>();

        filter_query.insert("occupancy".to_string());

        if let Some((current_subcmd, arg_matches)) = arg_matches.subcommand() {
            match current_subcmd {
                "file" => {
                    return ParsedArguments {
                        file: arg_matches
                            .get_one::<String>("DOCUMENT")
                            .cloned(),
                        is_debug,
                        filter_query,
                        ..Default::default()
                    }
                }
                "folder" => {
                    return ParsedArguments {
                        folder: arg_matches
                            .get_one::<String>("FOLDER")
                            .cloned(),
                        is_debug,
                        filter_query,
                        ..Default::default()
                    }
                }

                "stdin" => {
                    return ParsedArguments {
                        is_stdin: true,
                        is_debug,
                        filter_query,
                        ..Default::default()
                    }
                }
                _ => unreachable!(),
            }
        }
        unreachable!()
    }
}
