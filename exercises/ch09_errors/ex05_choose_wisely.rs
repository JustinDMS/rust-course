//! # Choosing the right tool
//!
//! Concept: `unwrap_or` for a sensible default, `expect` with a message
//! for an assumption that must hold, and `?` for an error the caller
//! should see. Each function below uses the wrong one.
//!
//! Book: 9.3 "To panic! or Not to panic!"
//!
//! Your job: make the tests pass. The function signatures are correct and
//! must stay as they are.

use std::num::ParseIntError;

/// The port to listen on: the `PORT` setting if present and valid,
/// otherwise 8080. A bad or missing setting is not an error.
fn port_from_setting(setting: Option<&str>) -> u16 {
    setting.unwrap().parse().unwrap()
}

/// The built-in default map, which is embedded in the binary and therefore
/// always present. If it is ever `None`, that is a bug in the build, and
/// the panic message must say `"built-in default map is missing"`.
fn default_map(embedded: Option<&str>) -> &str {
    embedded.unwrap()
}

/// The total of all values. A value that fails to parse is an error the
/// caller must see, not something to skip quietly.
fn total(values: &[&str]) -> Result<u32, ParseIntError> {
    let mut sum = 0;
    for v in values {
        sum += v.parse::<u32>().unwrap_or(0);
    }
    Ok(sum)
}

#[test]
fn ports_fall_back_to_8080() {
    assert_eq!(port_from_setting(Some("9000")), 9000);
    assert_eq!(port_from_setting(Some("lots")), 8080);
    assert_eq!(port_from_setting(None), 8080);
}

#[test]
fn default_map_is_returned() {
    assert_eq!(default_map(Some("#..#")), "#..#");
}

#[test]
#[should_panic(expected = "built-in default map is missing")]
fn missing_default_map_is_a_bug() {
    default_map(None);
}

#[test]
fn totals_report_bad_values() {
    assert_eq!(total(&["1", "2", "3"]), Ok(6));
    assert_eq!(total(&[]), Ok(0));
    assert!(total(&["1", "two", "3"]).is_err());
}
