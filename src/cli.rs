use clap::{arg, Command};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Cli {
    pub cmd: Command,
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
            .arg(arg!(-d --debug ...  "Turns on debugging and logging to file"))
            .arg(arg!(-o --outputFolderSuffix <OUTPUT_FOLDER_SUFFIX> "The output folder suffix"));

        Self { cmd }
    }
}
