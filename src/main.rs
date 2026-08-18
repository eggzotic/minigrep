use minigrep::search;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

mod config;
fn main() {
    let config = config::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let file_handle = File::open(&config.file_path)?;
    let content_reader = BufReader::new(file_handle);

    let results = search(config.ignore_case(), &config.query, content_reader.lines());

    let mut lines_matched = 0;
    for line in results {
        lines_matched += 1;
        println!("{}", line?);
    }
    eprintln!("Total lines matched: {lines_matched}");
    Ok(())
}
