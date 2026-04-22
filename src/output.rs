use crate::search::Match;
use colored::Colorize;

pub fn print_match(m: &Match, pattern: &str) {
    if let Some(start) = m.line.to_lowercase().find(&pattern.to_lowercase()) {
        let end = start + pattern.len();
        let colored = pattern.red().to_string();
        println!(
            "{}: {}",
            m.line_number,
            format!("{}{}{}", &m.line[..start], colored, &m.line[end..])
        );
    }
}
