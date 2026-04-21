#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub file_path: String,
    pub count_only: bool,  // flag --count
    pub ignore_case: bool, // flag -i
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("Usage: ferrogrep <pattern> <file>".to_string());
        }

        let mut positional: Vec<String> = Vec::new();
        let mut count_only = false;
        let mut ignore_case = false;

        for arg in args[1..].iter() {
            if arg.starts_with("--count") {
                count_only = true;
            } else if arg.starts_with("-i") {
                ignore_case = true;
            } else {
                positional.push(arg.clone());
            }
        }

        let pattern = positional
            .get(0)
            .cloned()
            .ok_or("Usage: ferrogrep <pattern> <file>".to_string())?;

        let file_path = positional
            .get(1)
            .cloned()
            .ok_or("Usage: ferrogrep <pattern> <file>".to_string())?;

        Ok(Config {
            count_only,
            ignore_case,
            pattern,
            file_path,
        })
    }
}
