mod config;
mod output;
mod search;
use config::Config;
use output::print_match;
use search::search_reader;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        std::process::exit(1);
    });

    let reader: Box<dyn BufRead> = match &config.file_path {
        Some(path) => {
            let file = File::open(path).unwrap_or_else(|e| {
                eprintln!("Error abriendo archivo: {e}");
                std::process::exit(1);
            });
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin())),
    };

    let matches = search_reader(reader, &config).unwrap_or_else(|err| {
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
