mod config;
mod output;
mod search;
use config::Config;
use output::print_match;
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

    if config.count_only {
        println!("Total matches: {}", matches.len())
    } else {
        for m in &matches {
            print_match(m, &config.pattern);
        }
    }
}
