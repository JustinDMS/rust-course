//! Stage 4: ignoring case. Edit `projects/minigrep/src/lib.rs`.

use minigrep::{Config, run, search_case_insensitive};

const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

fn poem() -> String {
    concat!(env!("CARGO_MANIFEST_DIR"), "/poem.txt").to_string()
}

#[test]
fn case_insensitive() {
    assert_eq!(search_case_insensitive("rUsT", CONTENTS), vec!["Rust:", "Trust me."]);
}

#[test]
fn run_honours_ignore_case() {
    let shouting = Config { query: String::from("FROG"), file_path: poem(), ignore_case: true };
    assert_eq!(run(shouting).unwrap(), vec!["How public, like a frog"]);

    let strict = Config { query: String::from("FROG"), file_path: poem(), ignore_case: false };
    assert!(run(strict).unwrap().is_empty());
}
