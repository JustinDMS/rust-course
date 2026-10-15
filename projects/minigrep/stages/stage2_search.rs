//! Stage 2: the search function, test-first. Edit `projects/minigrep/src/lib.rs`.

use minigrep::search;

const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

#[test]
fn one_result() {
    assert_eq!(vec!["safe, fast, productive."], search("duct", CONTENTS));
}

#[test]
fn case_sensitive() {
    assert_eq!(search("Duct", CONTENTS), vec!["Duct tape."]);
    assert!(search("DUCT", CONTENTS).is_empty());
}

#[test]
fn several_results_in_order() {
    assert_eq!(search("t", CONTENTS), vec!["Rust:", "safe, fast, productive.", "Pick three.", "Duct tape."]);
}

#[test]
fn results_borrow_from_contents() {
    let contents = String::from("alpha\nbeta");
    let results = search("alpha", &contents);
    assert_eq!(results[0].as_ptr(), contents.as_ptr());
}
