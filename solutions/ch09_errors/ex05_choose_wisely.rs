//! # Choosing the right tool (solution)
//!
//! A default for the port (`and_then` + `ok` + `unwrap_or`), an `expect`
//! with a message for the impossible case, and `?` to surface the parse
//! error.

use std::num::ParseIntError;

/// The port to listen on, defaulting to 8080.
fn port_from_setting(setting: Option<&str>) -> u16 {
    setting.and_then(|s| s.parse().ok()).unwrap_or(8080)
}

/// The built-in default map, which is always present.
fn default_map(embedded: Option<&str>) -> &str {
    embedded.expect("built-in default map is missing")
}

/// The total of all values; a bad value is the caller's problem.
fn total(values: &[&str]) -> Result<u32, ParseIntError> {
    let mut sum = 0;
    for v in values {
        sum += v.parse::<u32>()?;
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
