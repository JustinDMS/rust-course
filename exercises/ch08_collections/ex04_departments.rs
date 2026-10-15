//! # Employees and departments
//!
//! Concept: a `HashMap<String, Vec<String>>`, the entry API, parsing
//! simple commands, and sorting for stable output.
//!
//! Book: 8.3 (summary exercise)
//!
//! Commands look like `"Add Sally to Engineering"`. Anything else is
//! rejected.
//!
//! Your job: implement the three methods.

use std::collections::HashMap;

#[derive(Default)]
struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    /// Handles `"Add <name> to <department>"`. Returns `false` (and changes
    /// nothing) for any other shape of command.
    /// Hint: `command.split_whitespace().collect::<Vec<_>>()` and match on
    /// the slice: `[\"Add\", name, \"to\", dept]`.
    fn add(&mut self, command: &str) -> bool {
        todo!()
    }

    /// Everyone in a department, sorted alphabetically. Empty if the
    /// department does not exist.
    fn people_in(&self, department: &str) -> Vec<String> {
        todo!()
    }

    /// Every department with its sorted people, departments sorted too.
    fn all(&self) -> Vec<(String, Vec<String>)> {
        todo!()
    }
}

#[test]
fn adding_people() {
    let mut c = Company::default();
    assert!(c.add("Add Sally to Engineering"));
    assert!(c.add("Add Amir to Sales"));
    assert!(c.add("Add Bob to Engineering"));
    assert!(!c.add("Remove Bob from Engineering"));
    assert!(!c.add("Add Bob"));
    assert!(!c.add("Add Bob to"));
    assert_eq!(c.people_in("Engineering"), vec!["Bob", "Sally"]);
    assert_eq!(c.people_in("Sales"), vec!["Amir"]);
    assert!(c.people_in("Legal").is_empty());
}

#[test]
fn listing_everything() {
    let mut c = Company::default();
    c.add("Add Zed to Sales");
    c.add("Add Amy to Sales");
    c.add("Add Kim to Engineering");
    assert_eq!(
        c.all(),
        vec![
            (String::from("Engineering"), vec![String::from("Kim")]),
            (String::from("Sales"), vec![String::from("Amy"), String::from("Zed")]),
        ]
    );
    assert!(Company::default().all().is_empty());
}
