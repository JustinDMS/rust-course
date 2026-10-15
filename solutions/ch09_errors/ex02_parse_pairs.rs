//! # Result with a String error (solution)

fn parse_pair(s: &str) -> Result<(i32, i32), String> {
    let Some((left, right)) = s.split_once(',') else {
        return Err(format!("missing comma in '{s}'"));
    };
    let x = parse_part(left)?;
    let y = parse_part(right)?;
    Ok((x, y))
}

fn parse_part(part: &str) -> Result<i32, String> {
    let part = part.trim();
    part.parse().map_err(|_| format!("bad number '{part}'"))
}

fn parse_pairs(text: &str) -> Result<Vec<(i32, i32)>, String> {
    let mut pairs = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        pairs.push(parse_pair(line)?);
    }
    Ok(pairs)
}

fn sum_pairs(text: &str) -> Result<(i32, i32), String> {
    let mut sum = (0, 0);
    for (x, y) in parse_pairs(text)? {
        sum.0 += x;
        sum.1 += y;
    }
    Ok(sum)
}

#[test]
fn pairs_parse() {
    assert_eq!(parse_pair("3,4"), Ok((3, 4)));
    assert_eq!(parse_pair(" -1 , 20 "), Ok((-1, 20)));
}

#[test]
fn pair_errors_are_descriptive() {
    assert_eq!(parse_pair("34"), Err(String::from("missing comma in '34'")));
    assert_eq!(parse_pair("3,x"), Err(String::from("bad number 'x'")));
    assert_eq!(parse_pair("a, 4"), Err(String::from("bad number 'a'")));
}

#[test]
fn many_pairs() {
    assert_eq!(parse_pairs("1,2\n\n3,4\n"), Ok(vec![(1, 2), (3, 4)]));
    assert_eq!(parse_pairs(""), Ok(vec![]));
    assert_eq!(parse_pairs("1,2\n3;4"), Err(String::from("missing comma in '3;4'")));
}

#[test]
fn sums() {
    assert_eq!(sum_pairs("1,2\n3,4\n10,20"), Ok((14, 26)));
    assert_eq!(sum_pairs(""), Ok((0, 0)));
    assert_eq!(sum_pairs("1,2\nq,4"), Err(String::from("bad number 'q'")));
}
