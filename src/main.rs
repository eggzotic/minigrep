use minigrep::search;
use std::{env, error::Error, fs, process};

mod config;
fn main() {
    // let args: Vec<String> = env::args().collect();
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
    let contents = fs::read_to_string(&config.file_path)?;
    let results = search(config.ignore_case(), &config.query, &contents);

    for line in results {
        println!("{line}");
    }
    Ok(())
}
