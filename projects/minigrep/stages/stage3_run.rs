//! Stage 3: reading the file. Edit `projects/minigrep/src/lib.rs`.

use minigrep::{Config, run};

fn poem() -> String {
    concat!(env!("CARGO_MANIFEST_DIR"), "/poem.txt").to_string()
}

#[test]
fn run_returns_matching_lines() {
    let config = Config { query: String::from("body"), file_path: poem(), ignore_case: false };
    let lines = run(config).unwrap();
    assert_eq!(
        lines,
        vec!["I'm nobody! Who are you?", "Are you nobody, too?", "How dreary to be somebody!"]
    );
}

#[test]
fn run_with_no_matches_is_ok_and_empty() {
    let config = Config { query: String::from("metroid"), file_path: poem(), ignore_case: false };
    assert_eq!(run(config).unwrap(), Vec::<String>::new());
}

#[test]
fn run_reports_a_missing_file() {
    let config = Config { query: String::from("x"), file_path: String::from("does-not-exist.txt"), ignore_case: false };
    assert!(run(config).is_err());
}
