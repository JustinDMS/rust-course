//! # A pickup log (solution)

use std::collections::HashMap;

fn count_pickups(log: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for line in log.lines() {
        let name = line.trim();
        if name.is_empty() {
            continue;
        }
        *counts.entry(name.to_string()).or_insert(0) += 1;
    }
    counts
}

fn ranked(counts: &HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut entries: Vec<(String, u32)> = Vec::new();
    for (name, &count) in counts {
        entries.push((name.clone(), count));
    }
    // Highest count first; equal counts alphabetically.
    entries.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    entries
}

fn favourite(log: &str) -> Option<(String, u32)> {
    let counts = count_pickups(log);
    ranked(&counts).into_iter().next()
}

const LOG: &str = "missile\nenergy\n\nmissile\n  power bomb \nenergy\nmissile\n";

#[test]
fn counting() {
    let counts = count_pickups(LOG);
    assert_eq!(counts.len(), 3);
    assert_eq!(counts["missile"], 3);
    assert_eq!(counts["energy"], 2);
    assert_eq!(counts["power bomb"], 1);
    assert!(count_pickups("\n\n").is_empty());
}

#[test]
fn ranking() {
    let counts = count_pickups(LOG);
    assert_eq!(
        ranked(&counts),
        vec![
            (String::from("missile"), 3),
            (String::from("energy"), 2),
            (String::from("power bomb"), 1),
        ]
    );
    let tie = count_pickups("b\na\nb\na");
    assert_eq!(ranked(&tie), vec![(String::from("a"), 2), (String::from("b"), 2)]);
}

#[test]
fn favourites() {
    assert_eq!(favourite(LOG), Some((String::from("missile"), 3)));
    assert_eq!(favourite("b\na"), Some((String::from("a"), 1)));
    assert_eq!(favourite(""), None);
}
