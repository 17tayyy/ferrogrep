use crate::config::Config;
use std::io::BufRead;

pub struct Match {
    pub line_number: usize,
    pub line: String,
}

pub fn search_reader<R: BufRead>(reader: R, config: &Config) -> Result<Vec<Match>, String> {
    let content = reader.lines();

    let results = content
        .enumerate()
        .filter_map(|(i, line)| {
            let line = line.ok()?;
            let matches = if config.ignore_case {
                line.to_lowercase().contains(&config.pattern.to_lowercase())
            } else {
                line.contains(&config.pattern)
            };

            if matches {
                Some(Match {
                    line_number: i,
                    line: line.to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    Ok(results)
}
