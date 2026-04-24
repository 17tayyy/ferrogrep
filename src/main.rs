mod config;
mod output;
mod search;
use config::Config;
use output::print_match;
use search::search_reader;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;
use walkdir::WalkDir;

fn print_matches(matches: Vec<search::Match>, config: &Config, filename: Option<&str>) {
    if config.count_only {
        println!("Total matches: {}", matches.len());
    } else {
        for m in &matches {
            if let Some(name) = filename {
                print!("{name}:");
            }
            print_match(m, &config.pattern);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        exit(1);
    });

    if config.recursive {
        if let Some(path) = &config.file_path {
            let directory = WalkDir::new(path);

            for entry in directory {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };

                if !entry.file_type().is_file() {
                    continue;
                }

                let file = File::open(entry.path()).unwrap_or_else(|e| {
                    eprintln!("Error abriendo archivo: {e}");
                    exit(1);
                });

                let reader = Box::new(BufReader::new(file));

                let matches = search_reader(reader, &config).unwrap_or_else(|err| {
                    eprintln!("Error: {err}");
                    exit(1);
                });

                print_matches(matches, &config, entry.file_name().to_str());
            }
        } else {
            eprintln!("Error: directory needs to be specified when using --recursive");
            exit(1);
        }
    } else {
        let reader: Box<dyn BufRead> = match &config.file_path {
            Some(path) => {
                let file = File::open(path).unwrap_or_else(|e| {
                    eprintln!("Error abriendo archivo: {e}");
                    exit(1);
                });
                Box::new(BufReader::new(file))
            }
            None => {
                eprintln!("Reading from stdin... (Ctrl+D to end)");
                Box::new(BufReader::new(io::stdin()))
            }
        };
        let matches = search_reader(reader, &config).unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            exit(1);
        });
        print_matches(matches, &config, None);
    }
}
