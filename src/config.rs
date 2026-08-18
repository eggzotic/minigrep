use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Search for lines matching a pattern in a file")]
pub struct Config {
    /// Case-insensitive matching
    #[arg(
        short = 'i',
        long = "ignore-case",
        env = "IGNORE_CASE",
        action = clap::ArgAction::SetTrue,
        value_parser = clap::builder::BoolishValueParser::new(),
    )]
    pub ignore_case: bool,

    /// Prefix each matching line with its input-line-number
    #[arg(short = 'n', long = "line-number")]
    pub show_line_number: bool,

    /// The search-string
    pub query: String,

    /// The file to search
    pub file_path: PathBuf,
}
