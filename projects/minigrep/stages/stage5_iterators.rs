//! Stage 5 (chapter 13): building a Config from an iterator.
//! Edit `projects/minigrep/src/lib.rs`.

use minigrep::Config;

fn args(list: &[&str]) -> impl Iterator<Item = String> {
    list.iter().map(|s| s.to_string()).collect::<Vec<_>>().into_iter()
}

#[test]
fn build_from_iter_consumes_arguments() {
    let config = Config::build_from_iter(args(&["minigrep", "frog", "poem.txt"]), false).unwrap();
    assert_eq!(
        config,
        Config { query: String::from("frog"), file_path: String::from("poem.txt"), ignore_case: false }
    );
}

#[test]
fn build_from_iter_reports_missing_arguments() {
    assert_eq!(Config::build_from_iter(args(&["minigrep", "frog"]), false), Err("not enough arguments"));
    assert_eq!(Config::build_from_iter(args(&[]), false), Err("not enough arguments"));
}

#[test]
fn build_from_iter_works_with_env_args_shape() {
    // Anything that yields Strings works, including std::env::args().
    let owned = vec![String::from("prog"), String::from("q"), String::from("f")];
    let config = Config::build_from_iter(owned.into_iter(), true).unwrap();
    assert_eq!(config.query, "q");
    assert!(config.ignore_case);
}
