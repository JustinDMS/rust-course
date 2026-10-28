//! `course verify`: content CI. Every exercise must be red as shipped and
//! green with its solution.
//!
//! Two flavours:
//!
//! - `verify` copies the exercise and project crates into a scratch
//!   workspace and runs `cargo test` locally. Fast, works offline.
//! - `verify --online` posts each exercise to the Rust Playground exactly as
//!   the site does (one file, tests appended for project stages). Slower,
//!   but it checks the thing the learner actually runs: crate versions on
//!   the Playground, no access to files in the repository, and so on.

use crate::content::Content;
use crate::online;
use crate::runner::{self, RunOutcome, Status};
use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

pub struct Report {
    pub shipped_green: Vec<String>,
    pub solution_red: Vec<(String, Status, String)>,
    pub total: usize,
}

impl Report {
    pub fn ok(&self) -> bool {
        self.shipped_green.is_empty() && self.solution_red.is_empty()
    }

    pub fn print(&self) {
        println!();
        println!("{} exercises checked", self.total);
        for id in &self.shipped_green {
            println!("  !! {id} already passes as shipped");
        }
        for (id, status, detail) in &self.solution_red {
            println!("  !! {id} solution does not pass ({status:?}):");
            for line in detail.lines() {
                println!("       {line}");
            }
        }
        if self.ok() {
            println!("all good");
        }
    }
}

fn selected<'a>(content: &'a Content, only: Option<&str>) -> Result<Vec<&'a crate::content::Exercise>> {
    let exercises: Vec<_> = content
        .exercises
        .values()
        .filter(|e| only.is_none_or(|o| e.id.starts_with(o)))
        .collect();
    if exercises.is_empty() {
        bail!("no exercises match {only:?}");
    }
    Ok(exercises)
}

fn failure_detail(outcome: &RunOutcome) -> String {
    // Test failures are reported on stdout; compile errors on stderr.
    let detail = match outcome.status {
        Status::Failed | Status::RequirementsUnmet if !outcome.stdout.trim().is_empty() => &outcome.stdout,
        _ if outcome.stderr.trim().is_empty() => &outcome.stdout,
        _ => &outcome.stderr,
    };
    let detail = detail
        .lines()
        .filter(|l| !l.trim().is_empty())
        .take(40)
        .collect::<Vec<_>>()
        .join("\n");
    let checks = outcome
        .checks
        .iter()
        .filter(|c| !c.ok)
        .map(|c| format!("unmet: {}", c.label))
        .collect::<Vec<_>>()
        .join("\n");
    format!("{detail}\n{checks}")
}

// ---------------------------------------------------------------------------
// Online: through the Playground, as the site does it
// ---------------------------------------------------------------------------

pub fn verify_online(content: &Content, only: Option<&str>) -> Result<Report> {
    let exercises = selected(content, only)?;
    let playground = online::Playground::new();
    let mut shipped_green = Vec::new();
    let mut solution_red = Vec::new();

    println!("== Playground: shipped must NOT pass, solution MUST pass ({})", exercises.len());
    for ex in &exercises {
        let assembled = online::assemble(ex, &content.root)?;
        let tests = assembled.tests.as_deref();

        let shipped = online::run(&playground, ex, &assembled.starter, tests)?;
        let mark = if shipped.passed() { "GREEN (bad)" } else { "red (good)" };
        println!("  {:<14} shipped  {:<12} {:?} {}ms", ex.id, mark, shipped.status, shipped.duration_ms);
        if shipped.passed() {
            shipped_green.push(ex.id.clone());
        }

        let solved = online::run(&playground, ex, &assembled.solution, tests)?;
        let mark = if solved.passed() { "green (good)" } else { "RED (bad)" };
        println!("  {:<14} solution {:<12} {:?} {}ms", "", mark, solved.status, solved.duration_ms);
        if !solved.passed() {
            solution_red.push((ex.id.clone(), solved.status, failure_detail(&solved)));
        }
    }

    Ok(Report {
        shipped_green,
        solution_red,
        total: exercises.len(),
    })
}

// ---------------------------------------------------------------------------
// Local: cargo test in a scratch workspace
// ---------------------------------------------------------------------------

pub async fn verify(content: &Content, only: Option<&str>) -> Result<Report> {
    let root = &content.root;
    let scratch = std::env::temp_dir().join("rustcourse-verify");
    let target_dir = root.join("target").join("verify");
    if scratch.exists() {
        std::fs::remove_dir_all(&scratch)
            .with_context(|| format!("clearing {}", scratch.display()))?;
    }
    std::fs::create_dir_all(&scratch)?;

    if root.join("Cargo.lock").exists() {
        std::fs::copy(root.join("Cargo.lock"), scratch.join("Cargo.lock"))?;
    }
    // Cargo rejects a member glob that matches nothing, so only list the
    // directories that actually hold crates.
    let mut members = Vec::new();
    for dir in ["exercises", "projects"] {
        let from = root.join(dir);
        let has_crates = from.is_dir()
            && std::fs::read_dir(&from)?
                .any(|e| e.map(|e| e.path().join("Cargo.toml").exists()).unwrap_or(false));
        if has_crates {
            copy_tree(&from, &scratch.join(dir))?;
            members.push(format!("\"{dir}/*\""));
        }
    }
    std::fs::write(
        scratch.join("Cargo.toml"),
        format!(
            "[workspace]\nresolver = \"3\"\nmembers = [{}]\n\n[workspace.package]\nedition = \"2024\"\npublish = false\n",
            members.join(", ")
        ),
    )?;

    let exercises = selected(content, only)?;

    println!("== Phase 1: exercises as shipped must NOT pass ({})", exercises.len());
    let mut shipped_green = Vec::new();
    for ex in &exercises {
        let outcome = runner::run_exercise(&scratch, ex, Some(&target_dir)).await?;
        let mark = if outcome.passed() { "GREEN (bad)" } else { "red (good)" };
        println!("  {:<14} {:<16} {:?} {}ms", ex.id, mark, outcome.status, outcome.duration_ms);
        if outcome.passed() {
            shipped_green.push(ex.id.clone());
        }
    }

    println!("== Overlaying solutions");
    let mut copied = std::collections::HashSet::new();
    for ex in &exercises {
        let solution = ex.solution_path(root);
        let rel = ex
            .file_path(root)
            .strip_prefix(root)
            .map(Path::to_path_buf)
            .context("exercise file outside the course root")?;
        let dest = scratch.join(&rel);
        if copied.insert(dest.clone()) {
            if !solution.exists() {
                bail!("{}: solution file {} is missing", ex.id, solution.display());
            }
            copy_fresh(&solution, &dest)?;
        }
    }
    // A project's solution may span several files: overlay
    // solutions/<name>/src onto projects/<name>/src when present.
    let solutions_dir = root.join("solutions");
    if solutions_dir.exists() {
        for entry in std::fs::read_dir(&solutions_dir)? {
            let entry = entry?;
            let src = entry.path().join("src");
            if !src.is_dir() {
                continue;
            }
            for kind in ["projects", "exercises"] {
                let crate_dir = scratch.join(kind).join(entry.file_name());
                if crate_dir.is_dir() {
                    copy_tree(&src, &crate_dir.join("src"))?;
                }
            }
        }
    }

    println!("== Phase 2: exercises with solutions MUST pass");
    let mut solution_red = Vec::new();
    for ex in &exercises {
        let outcome = runner::run_exercise(&scratch, ex, Some(&target_dir)).await?;
        let mark = if outcome.passed() { "green (good)" } else { "RED (bad)" };
        println!("  {:<14} {:<16} {:?} {}ms", ex.id, mark, outcome.status, outcome.duration_ms);
        if !outcome.passed() {
            solution_red.push((ex.id.clone(), outcome.status, failure_detail(&outcome)));
        }
    }

    Ok(Report {
        shipped_green,
        solution_red,
        total: exercises.len(),
    })
}

fn copy_tree(from: &Path, to: &Path) -> Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from).with_context(|| format!("reading {}", from.display()))? {
        let entry = entry?;
        let name = entry.file_name();
        if name == "target" {
            continue;
        }
        let src: PathBuf = entry.path();
        let dst = to.join(&name);
        if src.is_dir() {
            copy_tree(&src, &dst)?;
        } else {
            copy_fresh(&src, &dst)?;
        }
    }
    Ok(())
}

/// Copies a file and stamps it with the current time. `std::fs::copy` keeps
/// the source's modification time on Windows, and cargo decides whether to
/// rebuild by mtime, so an overlaid solution with an old timestamp would be
/// ignored and the stale test binary re-run.
fn copy_fresh(src: &Path, dst: &Path) -> Result<()> {
    std::fs::copy(src, dst)
        .with_context(|| format!("copying {} to {}", src.display(), dst.display()))?;
    let file = std::fs::OpenOptions::new().write(true).open(dst)?;
    file.set_modified(std::time::SystemTime::now())
        .with_context(|| format!("touching {}", dst.display()))?;
    Ok(())
}
