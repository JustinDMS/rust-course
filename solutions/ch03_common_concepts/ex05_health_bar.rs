//! # A health bar (solution)

/// Health as a whole-number percentage, rounded down.
fn percent(hp: u32, max: u32) -> u32 {
    let hp = if hp > max { max } else { hp };
    hp * 100 / max
}

/// `[#####-----] 50%`
fn health_bar(hp: u32, max: u32, width: usize) -> String {
    let hp = if hp > max { max } else { hp };
    let filled = width * hp as usize / max as usize;
    let bar = "#".repeat(filled) + &"-".repeat(width - filled);
    format!("[{bar}] {}%", percent(hp, max))
}

/// A word for how the player is doing.
fn status(hp: u32, max: u32) -> &'static str {
    let p = percent(hp, max);
    if p == 100 {
        "full"
    } else if p >= 50 {
        "hurt"
    } else if p >= 25 {
        "wounded"
    } else {
        "critical"
    }
}

#[test]
fn percent_rounds_down() {
    assert_eq!(percent(1, 3), 33);
    assert_eq!(percent(2, 3), 66);
    assert_eq!(percent(0, 99), 0);
    assert_eq!(percent(99, 99), 100);
    assert_eq!(percent(150, 99), 100);
}

#[test]
fn bars() {
    assert_eq!(health_bar(50, 100, 10), "[#####-----] 50%");
    assert_eq!(health_bar(100, 100, 4), "[####] 100%");
    assert_eq!(health_bar(0, 100, 4), "[----] 0%");
    assert_eq!(health_bar(33, 99, 6), "[##----] 33%");
    assert_eq!(health_bar(1, 100, 10), "[----------] 1%");
}

#[test]
fn statuses() {
    assert_eq!(status(99, 99), "full");
    assert_eq!(status(98, 99), "hurt");
    assert_eq!(status(50, 100), "hurt");
    assert_eq!(status(49, 100), "wounded");
    assert_eq!(status(25, 100), "wounded");
    assert_eq!(status(24, 100), "critical");
    assert_eq!(status(0, 100), "critical");
}
