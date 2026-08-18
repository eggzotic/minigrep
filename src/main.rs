use clap::Parser;
use minigrep::search;

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

mod config;

fn main() {
    let config = config::Config::parse();

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let file_handle = File::open(&config.file_path)?;
    let content_reader = BufReader::new(file_handle);

    let results = search(config.ignore_case, &config.query, content_reader.lines());

    let mut lines_matched = 0;
    for entry in results {
        let (line_number, line) = entry?;
        lines_matched += 1;

        if config.show_line_number {
            println!("{line_number}: {line}");
        } else {
            println!("{line}");
        }
    }
    eprintln!("Total lines matched: {lines_matched}");
    Ok(())
}
