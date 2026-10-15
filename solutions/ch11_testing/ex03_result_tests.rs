//! # Tests that return Result (solution)

fn parse_version(s: &str) -> Result<(u32, u32, u32), String> {
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return Err(format!("expected 3 parts, got {}", parts.len()));
    }
    let mut numbers = [0u32; 3];
    for (i, part) in parts.iter().enumerate() {
        numbers[i] = part.parse().map_err(|_| format!("bad number '{part}'"))?;
    }
    Ok((numbers[0], numbers[1], numbers[2]))
}

#[test]
fn parses_a_normal_version() -> Result<(), String> {
    let v = parse_version("1.2.3")?;
    assert_eq!(v, (1, 2, 3));
    Ok(())
}

#[test]
fn parses_big_numbers() -> Result<(), String> {
    let (major, minor, patch) = parse_version("10.200.3000")?;
    assert_eq!(major + minor + patch, 3210);
    Ok(())
}

#[test]
fn wrong_part_count_is_an_error() {
    assert_eq!(parse_version("1.2"), Err(String::from("expected 3 parts, got 2")));
    assert_eq!(parse_version("1.2.3.4"), Err(String::from("expected 3 parts, got 4")));
}

#[test]
fn bad_numbers_are_errors() {
    assert_eq!(parse_version("1.x.3"), Err(String::from("bad number 'x'")));
}

#[test]
fn parses_zero() -> Result<(), String> {
    assert_eq!(parse_version("0.0.0")?, (0, 0, 0));
    Ok(())
}
