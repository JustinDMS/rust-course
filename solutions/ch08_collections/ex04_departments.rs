//! # Employees and departments (solution)

use std::collections::HashMap;

#[derive(Default)]
struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn add(&mut self, command: &str) -> bool {
        let words: Vec<&str> = command.split_whitespace().collect();
        match words.as_slice() {
            ["Add", name, "to", dept] => {
                self.departments
                    .entry(dept.to_string())
                    .or_default()
                    .push(name.to_string());
                true
            }
            _ => false,
        }
    }

    fn people_in(&self, department: &str) -> Vec<String> {
        let mut people = match self.departments.get(department) {
            Some(people) => people.clone(),
            None => Vec::new(),
        };
        people.sort();
        people
    }

    fn all(&self) -> Vec<(String, Vec<String>)> {
        let mut out = Vec::new();
        for dept in self.departments.keys() {
            out.push((dept.clone(), self.people_in(dept)));
        }
        out.sort();
        out
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
