//! # Write the tests yourself (solution)

/// Keeps `hp` within `0..=max`. Panics if `max` is 0.
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

#[test]
fn hp_inside_the_range_is_unchanged() {
    assert_eq!(clamp_hp(50, 100), 50);
    assert_eq!(clamp_hp(0, 100), 0);
    assert_eq!(clamp_hp(100, 100), 100);
}

#[test]
fn hp_is_clamped_at_both_ends() {
    assert_eq!(clamp_hp(-20, 100), 0);
    assert_eq!(clamp_hp(999, 100), 100);
}

#[test]
fn levels_change_exactly_at_the_thresholds() {
    assert_eq!(level_for_xp(0), 1);
    assert_eq!(level_for_xp(99), 1);
    assert_eq!(level_for_xp(100), 2);
    assert_eq!(level_for_xp(299), 2);
    assert_eq!(level_for_xp(300), 3);
    assert_eq!(level_for_xp(599), 3);
    assert_eq!(level_for_xp(600), 4);
    assert_eq!(level_for_xp(100_000), 4);
}

#[test]
#[should_panic(expected = "max hp must be positive")]
fn zero_max_hp_is_a_bug() {
    clamp_hp(10, 0);
}
