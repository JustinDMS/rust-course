//! # minigrep, as a library
//!
//! Stage 1 (chapter 12): `Config::build_with` and `Config::build`
//! Stage 2: `search`
//! Stage 3: `run`
//! Stage 4: `search_case_insensitive`, and `run` honouring `ignore_case`
//! Stage 5 (chapter 13): `Config::build_from_iter`
//!
//! Book: Chapter 12 "An I/O Project" and 13.4 "Improving Our I/O Project"

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
    /// Builds a config from the raw arguments (`args[0]` is the program
    /// path, then the query, then the file path). `ignore_case` is given
    /// directly so it can be tested without touching the environment.
    ///
    /// Errors with `"not enough arguments"` when fewer than two arguments
    /// follow the program path.
    pub fn build_with(args: &[String], ignore_case: bool) -> Result<Config, &'static str> {
        todo!()
    }

    /// Like `build_with`, with `ignore_case` taken from whether the
    /// `IGNORE_CASE` environment variable is set.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Config::build_with(args, ignore_case)
    }

    /// Stage 5: the same as `build_with`, but reading from an iterator of
    /// arguments with `next()` instead of indexing a slice and cloning.
    /// The first item is the program path and is skipped.
    pub fn build_from_iter(
        args: impl Iterator<Item = String>,
        ignore_case: bool,
    ) -> Result<Config, &'static str> {
        todo!()
    }
}

/// Reads the file named in the config and returns the matching lines.
pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    todo!()
}

/// Every line of `contents` that contains `query`, case-sensitively.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    todo!()
}

/// Every line of `contents` that contains `query`, ignoring case.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    todo!()
}
