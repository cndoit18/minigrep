use std::{env, error::Error, fs};
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        Ok(Config {
            query: query,
            file_path: file_path,
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contexts = fs::read_to_string(&config.file_path)?;
    for (line, context) in if config.ignore_case {
        search_case_insensitive(&config.query, &contexts)
    } else {
        search(&config.query, &contexts)
    } {
        println!("{line}:{context}")
    }
    Ok(())
}

fn search<'a>(query: &str, contexts: &'a str) -> Vec<(usize, &'a str)> {
    contexts
        .lines()
        .enumerate()
        .filter(|(_, context)| context.contains(&query))
        .map(|(line, context)| (line + 1, context))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contexts: &'a str) -> Vec<(usize, &'a str)> {
    contexts
        .lines()
        .enumerate()
        .filter(|(_, context)| context.to_lowercase().contains(&query.to_lowercase()))
        .map(|(line, context)| (line + 1, context))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec![(2, "safe, fast, productive.")],
            search(query, contents)
        );
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![(1, "Rust:"), (4, "Trust me.")],
            search_case_insensitive(query, contents)
        );
    }
}
