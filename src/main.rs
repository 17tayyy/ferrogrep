mod config;
mod search;
use config::Config;
use search::search;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        std::process::exit(1);
    });

    let matches = search(&config).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        std::process::exit(1);
    });

    for m in &matches {
        println!("{}: {}", m.line_number + 1, m.line);
    }
}
