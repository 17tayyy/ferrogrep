use crate::config::Config;
use std::fs;

pub struct Match {
    pub line_number: usize,
    pub line: String,
}

pub fn search(config: &Config) -> Result<Vec<Match>, String> {
    let content =
        fs::read_to_string(&config.file_path).map_err(|e| format!("Error leyendo archivo: {e}"))?;

    let results = content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.contains(&config.pattern) {
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
