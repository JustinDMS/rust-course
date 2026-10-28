//! Stage 4: ignoring case.

use minigrep::{Config, run, search_case_insensitive};
use std::sync::LazyLock;

const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

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
    let path = std::env::temp_dir().join("rustcourse-minigrep-poem-stage4.txt");
    std::fs::write(&path, POEM).expect("writing the poem to a temporary file");
    path.to_string_lossy().into_owned()
});

fn poem() -> String {
    POEM_PATH.clone()
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
