//! Runs `cargo test` for one exercise and classifies the outcome.

use crate::content::{Exercise, Requires};
use anyhow::{Context, Result};
use serde::Serialize;
use std::path::Path;
use std::process::Stdio;
use std::time::{Duration, Instant};
use tokio::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Passed,
    Failed,
    CompileError,
    Timeout,
    /// Tests passed but a `requires` check did not.
    RequirementsUnmet,
}

#[derive(Debug, Clone, Serialize)]
pub struct Check {
    pub label: String,
    pub ok: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunOutcome {
    pub status: Status,
    pub stdout: String,
    pub stderr: String,
    pub checks: Vec<Check>,
    pub duration_ms: u128,
    /// `cargo test -p ... --test ...` as shown to the learner.
    pub command: String,
}

impl RunOutcome {
    pub fn passed(&self) -> bool {
        self.status == Status::Passed
    }
}

pub const TIMEOUT: Duration = Duration::from_secs(180);

/// Runs the exercise's `[[test]]` target inside `workspace_root`.
///
/// `target_dir` overrides `CARGO_TARGET_DIR` (used by `verify` so it does not
/// fight with the learner's own builds).
pub async fn run_exercise(
    workspace_root: &Path,
    exercise: &Exercise,
    target_dir: Option<&Path>,
) -> Result<RunOutcome> {
    let args = [
        "test",
        "-p",
        exercise.crate_name.as_str(),
        "--test",
        exercise.target.as_str(),
        "--color",
        "never",
    ];
    let command = format!(
        "cargo test -p {} --test {}",
        exercise.crate_name, exercise.target
    );

    let mut cmd = Command::new("cargo");
    cmd.args(args)
        .current_dir(workspace_root)
        .env("CARGO_TERM_COLOR", "never")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    if let Some(dir) = target_dir {
        cmd.env("CARGO_TARGET_DIR", dir);
    }

    let started = Instant::now();
    let output = tokio::time::timeout(TIMEOUT, cmd.output()).await;
    let duration_ms = started.elapsed().as_millis();

    let output = match output {
        Ok(result) => result.context("spawning cargo")?,
        Err(_) => {
            return Ok(RunOutcome {
                status: Status::Timeout,
                stdout: String::new(),
                stderr: format!("cargo test did not finish within {} seconds", TIMEOUT.as_secs()),
                checks: Vec::new(),
                duration_ms,
                command,
            });
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let mut status = classify(output.status.success(), &stderr);

    let mut checks = Vec::new();
    if status == Status::Passed {
        let source = std::fs::read_to_string(exercise.file_path(workspace_root)).unwrap_or_default();
        checks = extra_checks(&exercise.requires, &source);
        if checks.iter().any(|c| !c.ok) {
            status = Status::RequirementsUnmet;
        }
    }

    Ok(RunOutcome {
        status,
        stdout,
        stderr,
        checks,
        duration_ms,
        command,
    })
}

/// Classifies cargo's exit status plus stderr.
pub fn classify(success: bool, stderr: &str) -> Status {
    if success {
        Status::Passed
    } else if stderr.contains("could not compile") || stderr.contains("error[E") || stderr.contains("error: expected") || stderr.contains("error: cannot") {
        Status::CompileError
    } else {
        Status::Failed
    }
}

/// Source-level requirements that `cargo test` cannot express (e.g. "write at
/// least three tests yourself").
pub fn extra_checks(req: &Requires, source: &str) -> Vec<Check> {
    let mut checks = Vec::new();
    if req.min_tests > 0 {
        let count = count_tests(source);
        checks.push(Check {
            label: format!("at least {} #[test] functions (found {count})", req.min_tests),
            ok: count >= req.min_tests,
        });
    }
    let code = strip_comments(source);
    for needle in &req.must_contain {
        checks.push(Check {
            label: format!("file uses `{needle}`"),
            ok: code.contains(needle),
        });
    }
    for needle in &req.must_not_contain {
        checks.push(Check {
            label: format!("file does not use `{needle}`"),
            ok: !code.contains(needle),
        });
    }
    checks
}

/// Drops `//` comments so that instructions may mention forbidden helpers.
/// (Does not understand `//` inside string literals; good enough here.)
pub fn strip_comments(source: &str) -> String {
    source
        .lines()
        .map(|l| match l.find("//") {
            Some(i) => &l[..i],
            None => l,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Counts `#[test]` attributes outside of `//` comments.
pub fn count_tests(source: &str) -> u32 {
    source
        .lines()
        .map(|l| l.trim_start())
        .filter(|l| !l.starts_with("//"))
        .filter(|l| l.starts_with("#[test]") || l.starts_with("#[test ") || l.starts_with("#[tokio::test"))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_outcomes() {
        assert_eq!(classify(true, ""), Status::Passed);
        assert_eq!(
            classify(false, "error[E0382]: borrow of moved value: `s`\nerror: could not compile `ch04_ownership`"),
            Status::CompileError
        );
        assert_eq!(
            classify(false, "test ex01::works ... FAILED\nfailures:\n  assertion `left == right` failed"),
            Status::Failed
        );
    }

    #[test]
    fn counts_tests_ignoring_comments() {
        let src = "#[test]\nfn a() {}\n// #[test]\nfn b() {}\n    #[test]\n    fn c() {}\n#[test_case]\nfn d(){}";
        assert_eq!(count_tests(src), 2);
    }

    #[test]
    fn requirement_checks() {
        let req = Requires {
            min_tests: 3,
            must_contain: vec!["should_panic".into()],
            must_not_contain: vec!["unwrap()".into()],
        };
        let checks = extra_checks(&req, "#[test]\nfn a(){}\n#[test]\nfn b(){}\n#[should_panic]\nfn c(){ x.unwrap() }");
        assert_eq!(checks.len(), 3);
        assert!(!checks[0].ok);
        assert!(checks[1].ok);
        assert!(!checks[2].ok);
    }
}

#[cfg(test)]
mod comment_tests {
    use super::*;

    #[test]
    fn forbidden_words_in_comments_are_ignored() {
        let req = Requires {
            must_not_contain: vec!["split".into()],
            ..Default::default()
        };
        let checks = extra_checks(&req, "//! do not use `split` here\nfn a() {} // split\n");
        assert!(checks[0].ok);
        let checks = extra_checks(&req, "fn a() { s.split(' '); }");
        assert!(!checks[0].ok);
    }
}
