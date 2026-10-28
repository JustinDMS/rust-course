//! Stage 3: reading the file.

use minigrep::{Config, run};
use std::sync::LazyLock;

/// Emily Dickinson's "I'm Nobody! Who are you?", the book's sample text.
const POEM: &str = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
";

/// The poem written to a temporary file, once; `run` needs a real path.
static POEM_PATH: LazyLock<String> = LazyLock::new(|| {
    let path = std::env::temp_dir().join("rustcourse-minigrep-poem.txt");
    std::fs::write(&path, POEM).expect("writing the poem to a temporary file");
    path.to_string_lossy().into_owned()
});

fn poem() -> String {
    POEM_PATH.clone()
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
