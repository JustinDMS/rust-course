//! Exercises as the Rust Playground sees them.
//!
//! The site has no server: an exercise is one library crate's `src/lib.rs`,
//! posted to `https://play.rust-lang.org/execute` with `tests: true`, which
//! runs `cargo test` on it. Ordinary exercises already carry their tests in
//! the file. A project stage edits the project's `lib.rs` and gets its
//! `stages/<target>.rs` integration test appended as a `#[cfg(test)]` module.
//!
//! `docs/app.js` mirrors `combined` and `classify` in JavaScript; keep them
//! in step.

use crate::content::Exercise;
use crate::runner::{self, RunOutcome, Status};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::{Duration, Instant};

const EXECUTE_URL: &str = "https://play.rust-lang.org/execute";

/// The three texts the site needs for one exercise.
pub struct Assembled {
    /// What the editor starts with (the shipped file).
    pub starter: String,
    /// Appended to the editor's text before running, when the tests live
    /// in a separate file (project stages). `None` when the file is
    /// self-contained.
    pub tests: Option<String>,
    /// The reference solution, in the same shape as `starter`.
    pub solution: String,
}

pub fn assemble(exercise: &Exercise, root: &Path) -> Result<Assembled> {
    let read = |path: std::path::PathBuf| {
        std::fs::read_to_string(&path).with_context(|| format!("{}: reading {}", exercise.id, path.display()))
    };
    let starter = read(exercise.file_path(root))?;
    let solution = read(exercise.solution_path(root))?;
    let tests = match exercise.tests_path(root) {
        Some(path) => Some(inline_tests(&exercise.crate_name, &exercise.target, &read(path)?)),
        None => None,
    };
    Ok(Assembled {
        starter,
        tests,
        solution,
    })
}

/// Turns a `stages/<target>.rs` integration test (which does
/// `use <crate>::Thing;`) into a test module that can sit at the bottom of
/// the library file itself (`use super::Thing;`).
pub fn inline_tests(crate_name: &str, target: &str, source: &str) -> String {
    let needle = format!("{crate_name}::");
    let mut out = String::new();
    out.push_str("// ---- Stage tests: appended when you press Run; not editable ----\n");
    out.push_str("#[cfg(test)]\nmod ");
    out.push_str(target);
    out.push_str(" {\n");
    // The body is deliberately not re-indented: the tests hold multi-line
    // string literals (poems, sample file contents) whose text would change.
    for line in source.lines() {
        if line.starts_with("//!") {
            continue;
        }
        out.push_str(&line.replace(&needle, "super::"));
        out.push('\n');
    }
    out.push_str("}\n");
    out
}

/// The code actually sent to the Playground.
pub fn combined(source: &str, tests: Option<&str>) -> String {
    match tests {
        Some(tests) => format!("{}\n\n{}", source.trim_end(), tests),
        None => source.to_string(),
    }
}

// ---------------------------------------------------------------------------
// The Playground API
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExecuteRequest<'a> {
    channel: &'a str,
    mode: &'a str,
    edition: &'a str,
    crate_type: &'a str,
    tests: bool,
    code: &'a str,
    backtrace: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteResponse {
    pub success: bool,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
}

pub struct Playground {
    agent: ureq::Agent,
}

impl Playground {
    pub fn new() -> Playground {
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(120)))
            .build();
        Playground {
            agent: config.into(),
        }
    }

    /// Runs `cargo test` on `code` as a library crate.
    pub fn test(&self, code: &str) -> Result<ExecuteResponse> {
        let body = ExecuteRequest {
            channel: "stable",
            mode: "debug",
            edition: "2024",
            crate_type: "lib",
            tests: true,
            code,
            backtrace: false,
        };
        let mut last_err = None;
        for attempt in 1..=3u64 {
            match self.agent.post(EXECUTE_URL).send_json(&body) {
                Ok(mut response) => {
                    return response
                        .body_mut()
                        .read_json::<ExecuteResponse>()
                        .context("decoding the playground's reply");
                }
                Err(e) => {
                    eprintln!("    playground request failed ({e}); retrying");
                    last_err = Some(e);
                    std::thread::sleep(Duration::from_secs(5 * attempt));
                }
            }
        }
        Err(anyhow::Error::new(last_err.expect("at least one attempt")).context("reaching play.rust-lang.org"))
    }
}

/// Same rules as the browser: a pass needs an exit status of 0 *and* the
/// test harness summary line; a compile error is recognised from stderr.
pub fn classify(response: &ExecuteResponse) -> Status {
    if response.success && response.stdout.contains("test result: ok") {
        Status::Passed
    } else {
        runner::classify(false, &response.stderr)
    }
}

/// Runs one exercise the way the site does and applies its extra checks to
/// the editable part only.
pub fn run(playground: &Playground, exercise: &Exercise, source: &str, tests: Option<&str>) -> Result<RunOutcome> {
    let code = combined(source, tests);
    let started = Instant::now();
    let response = playground.test(&code)?;
    let duration_ms = started.elapsed().as_millis();
    let mut status = classify(&response);
    let mut checks = Vec::new();
    if status == Status::Passed {
        checks = runner::extra_checks(&exercise.requires, source);
        if checks.iter().any(|c| !c.ok) {
            status = Status::RequirementsUnmet;
        }
    }
    Ok(RunOutcome {
        status,
        stdout: response.stdout,
        stderr: response.stderr,
        checks,
        duration_ms,
        command: "play.rust-lang.org/execute (tests)".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_tests_become_a_module() {
        let src = "//! Stage 1.\n\nuse guessing_game::parse_guess;\n\n#[test]\nfn parses() {\n    assert_eq!(parse_guess(\"1\"), Some(1));\n}\n";
        let out = inline_tests("guessing_game", "stage1_parse", src);
        assert!(out.contains("#[cfg(test)]\nmod stage1_parse {"));
        assert!(out.contains("\nuse super::parse_guess;\n"));
        assert!(!out.contains("//!"));
        assert!(out.trim_end().ends_with('}'));
    }

    #[test]
    fn classification_needs_the_summary_line() {
        let ok = ExecuteResponse {
            success: true,
            stdout: "running 1 test\ntest a ... ok\n\ntest result: ok. 1 passed".into(),
            stderr: String::new(),
        };
        assert_eq!(classify(&ok), Status::Passed);
        let failed = ExecuteResponse {
            success: false,
            stdout: "test a ... FAILED\ntest result: FAILED. 0 passed; 1 failed".into(),
            stderr: "error: test failed, to rerun pass `--lib`".into(),
        };
        assert_eq!(classify(&failed), Status::Failed);
        let compile = ExecuteResponse {
            success: false,
            stdout: String::new(),
            stderr: "error[E0382]: borrow of moved value\nerror: could not compile `playground`".into(),
        };
        assert_eq!(classify(&compile), Status::CompileError);
    }
}
