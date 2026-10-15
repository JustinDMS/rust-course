//! # Write the tests yourself
//!
//! Concept: deciding what to test. Good tests cover the normal case, the
//! edges, and the failure mode.
//!
//! Book: 11.1 and 11.3
//!
//! The two functions below are finished and (we claim) correct. There are
//! no tests. Write at least **four** `#[test]` functions:
//!
//! 1. `clamp_hp` with a value inside the range.
//! 2. `clamp_hp` with values beyond both ends.
//! 3. `level_for_xp` at the exact thresholds (0, 100, 300, 600) and just
//!    below them.
//! 4. A `#[should_panic(expected = "...")]` test for `clamp_hp` when
//!    `max` is 0.
//!
//! The checker counts `#[test]` attributes and looks for `should_panic`.

/// Keeps `hp` within `0..=max`. Panics if `max` is 0, because a character
/// with no health at all is a bug in the caller.
pub fn clamp_hp(hp: i64, max: u32) -> u32 {
    if max == 0 {
        panic!("max hp must be positive");
    }
    if hp < 0 {
        0
    } else if hp > max as i64 {
        max
    } else {
        hp as u32
    }
}

/// Level 1 below 100 XP, 2 below 300, 3 below 600, 4 from 600 upwards.
pub fn level_for_xp(xp: u32) -> u32 {
    const THRESHOLDS: [u32; 3] = [100, 300, 600];
    let mut level = 1;
    for t in THRESHOLDS {
        if xp >= t {
            level += 1;
        }
    }
    level
}

// Your tests go below.
