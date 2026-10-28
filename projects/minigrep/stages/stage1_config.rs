//! Stage 1: building a Config.

use minigrep::Config;

fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn build_with_takes_query_and_path() {
    let config = Config::build_with(&args(&["minigrep", "frog", "poem.txt"]), false).unwrap();
    assert_eq!(
        config,
        Config { query: String::from("frog"), file_path: String::from("poem.txt"), ignore_case: false }
    );
}

#[test]
fn build_with_passes_ignore_case_through() {
    let config = Config::build_with(&args(&["minigrep", "a", "b"]), true).unwrap();
    assert!(config.ignore_case);
}

#[test]
fn extra_arguments_are_ignored() {
    let config = Config::build_with(&args(&["minigrep", "a", "b", "c"]), false).unwrap();
    assert_eq!(config.query, "a");
    assert_eq!(config.file_path, "b");
}

#[test]
fn too_few_arguments_is_an_error() {
    assert_eq!(Config::build_with(&args(&["minigrep", "frog"]), false), Err("not enough arguments"));
    assert_eq!(Config::build_with(&args(&["minigrep"]), false), Err("not enough arguments"));
    assert_eq!(Config::build(&args(&[])), Err("not enough arguments"));
}
