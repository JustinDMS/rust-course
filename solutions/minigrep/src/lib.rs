//! # minigrep, as a library (solution)

use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build_with(args: &[String], ignore_case: bool) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Config::build_with(args, ignore_case)
    }

    pub fn build_from_iter(
        mut args: impl Iterator<Item = String>,
        ignore_case: bool,
    ) -> Result<Config, &'static str> {
        args.next(); // the program path

        let Some(query) = args.next() else {
            return Err("not enough arguments");
        };
        let Some(file_path) = args.next() else {
            return Err("not enough arguments");
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    Ok(results.into_iter().map(String::from).collect())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
