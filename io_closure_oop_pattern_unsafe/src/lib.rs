pub mod state_pattern;
pub mod trait_object;
pub mod raw_pointer;

pub use trait_object::build_ui;

use std::error::Error;
use std::fs;
use anyhow::Error as AnyErr;
use anyhow::anyhow;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build (args: &[String]) -> Result<Config, AnyErr> {
        if args.len() < 3 {
            return Err(anyhow!("not enough arg"));
        }

        let query = &args[1];
        let file_path = &args[2];
        // if use explicitly set case sentivity, use it
        // otherwise, use env var `IGNORE_CASE`
        // default is true
        let mut ignore_case = true;
        if args.len() == 4 {
            ignore_case = args[3].parse()?;
        } else {
            match env::var("IGNORE_CASE") {
                Ok(v) => {ignore_case = v.parse()?},
                Err(v) => {
                    match v {
                        env::VarError::NotPresent => {eprintln!("IGNORE_CASE is not set either from input nor from environment var. Leave it as default true")},
                        env::VarError::NotUnicode(_) => {},
                        _ => {},
                    }
                },
            }
        }

        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case: ignore_case
         })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect::<Vec<&str>>()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
