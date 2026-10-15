//! `progress.json`: what the learner has done, and the XP it earned.

use crate::content::Level;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Progress {
    #[serde(default)]
    pub xp: u32,
    #[serde(default)]
    pub cards: BTreeMap<String, CardProgress>,
    #[serde(default)]
    pub exercises: BTreeMap<String, ExerciseProgress>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardProgress {
    pub done: bool,
    pub attempts: u32,
    /// Correct on the first attempt (full XP) rather than later (half XP).
    pub first_try: bool,
    pub xp_awarded: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExerciseProgress {
    pub passed: bool,
    pub attempts: u32,
    pub hints_revealed: u32,
    pub solution_revealed: bool,
    pub xp_awarded: u32,
    #[serde(default)]
    pub last_status: String,
}

/// Result of recording an answer or a run: how much XP it added.
#[derive(Debug, Clone, Serialize)]
pub struct Award {
    pub xp_gained: u32,
    pub xp_total: u32,
    pub level_before: usize,
    pub level_after: usize,
}

impl Progress {
    pub fn load(path: &Path) -> Result<Progress> {
        if !path.exists() {
            return Ok(Progress::default());
        }
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading {}", path.display()))?;
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let text = serde_json::to_string_pretty(self)?;
        let tmp: PathBuf = path.with_extension("json.tmp");
        std::fs::write(&tmp, text).with_context(|| format!("writing {}", tmp.display()))?;
        std::fs::rename(&tmp, path).with_context(|| format!("replacing {}", path.display()))?;
        Ok(())
    }

    /// Records an answer to a read/code/predict/quiz card.
    /// Full XP on the first correct attempt, half afterwards, never twice.
    pub fn answer_card(&mut self, card_id: &str, card_xp: u32, correct: bool, levels: &[Level]) -> Award {
        let before = level_index(levels, self.xp);
        let entry = self.cards.entry(card_id.to_string()).or_default();
        let mut gained = 0;
        if !entry.done {
            entry.attempts += 1;
            if correct {
                entry.done = true;
                entry.first_try = entry.attempts == 1;
                gained = if entry.first_try { card_xp } else { card_xp.div_ceil(2) };
                entry.xp_awarded = gained;
                self.xp += gained;
            }
        }
        Award {
            xp_gained: gained,
            xp_total: self.xp,
            level_before: before,
            level_after: level_index(levels, self.xp),
        }
    }

    /// Records a test run. XP is awarded once, on the first pass, halved if
    /// the solution was revealed beforehand.
    pub fn record_run(&mut self, id: &str, exercise_xp: u32, passed: bool, status: &str, levels: &[Level]) -> Award {
        let before = level_index(levels, self.xp);
        let entry = self.exercises.entry(id.to_string()).or_default();
        entry.attempts += 1;
        entry.last_status = status.to_string();
        let mut gained = 0;
        if passed && !entry.passed {
            entry.passed = true;
            gained = if entry.solution_revealed { exercise_xp.div_ceil(2) } else { exercise_xp };
            entry.xp_awarded = gained;
            self.xp += gained;
        }
        Award {
            xp_gained: gained,
            xp_total: self.xp,
            level_before: before,
            level_after: level_index(levels, self.xp),
        }
    }

    pub fn reveal_hint(&mut self, id: &str, n: u32) {
        let entry = self.exercises.entry(id.to_string()).or_default();
        entry.hints_revealed = entry.hints_revealed.max(n);
    }

    pub fn reveal_solution(&mut self, id: &str) {
        let entry = self.exercises.entry(id.to_string()).or_default();
        entry.solution_revealed = true;
    }
}

/// Index into `levels` (sorted by xp ascending) of the level reached at `xp`.
pub fn level_index(levels: &[Level], xp: u32) -> usize {
    levels
        .iter()
        .enumerate()
        .filter(|(_, l)| l.xp <= xp)
        .map(|(i, _)| i)
        .last()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn levels() -> Vec<Level> {
        vec![
            Level { xp: 0, title: "Newcomer".into() },
            Level { xp: 100, title: "Hello World".into() },
            Level { xp: 300, title: "Cargo Pilot".into() },
        ]
    }

    #[test]
    fn level_index_picks_highest_reached() {
        let l = levels();
        assert_eq!(level_index(&l, 0), 0);
        assert_eq!(level_index(&l, 99), 0);
        assert_eq!(level_index(&l, 100), 1);
        assert_eq!(level_index(&l, 5000), 2);
    }

    #[test]
    fn card_xp_full_then_half_then_none() {
        let l = levels();
        let mut p = Progress::default();
        let a = p.answer_card("c1", 20, false, &l);
        assert_eq!(a.xp_gained, 0);
        let a = p.answer_card("c1", 20, true, &l);
        assert_eq!(a.xp_gained, 10);
        let a = p.answer_card("c1", 20, true, &l);
        assert_eq!(a.xp_gained, 0);
        assert_eq!(p.xp, 10);

        let a = p.answer_card("c2", 15, true, &l);
        assert_eq!(a.xp_gained, 15);
        assert!(p.cards["c2"].first_try);
        assert_eq!(p.xp, 25);
    }

    #[test]
    fn exercise_xp_once_and_halved_after_solution() {
        let l = levels();
        let mut p = Progress::default();
        p.record_run("e1", 40, false, "compile_error", &l);
        p.reveal_solution("e1");
        let a = p.record_run("e1", 40, true, "passed", &l);
        assert_eq!(a.xp_gained, 20);
        let a = p.record_run("e1", 40, true, "passed", &l);
        assert_eq!(a.xp_gained, 0);
        assert_eq!(p.exercises["e1"].attempts, 3);

        let a = p.record_run("e2", 100, true, "passed", &l);
        assert_eq!(a.xp_gained, 100);
        assert_eq!(a.level_before, 0);
        assert_eq!(a.level_after, 1);
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = std::env::temp_dir().join(format!("rustcourse-progress-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("progress.json");
        let mut p = Progress::default();
        p.answer_card("c1", 10, true, &levels());
        p.save(&path).unwrap();
        let q = Progress::load(&path).unwrap();
        assert_eq!(q.xp, 10);
        assert!(q.cards["c1"].done);
        std::fs::remove_dir_all(&dir).ok();
    }
}
