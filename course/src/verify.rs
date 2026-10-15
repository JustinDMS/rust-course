//! `course verify`: content CI. Copies the exercise and project crates into a
//! scratch workspace, checks that every exercise is red as shipped, overlays
//! the solutions, and checks that every exercise is then green.

use crate::content::Content;
use crate::runner::{self, Status};
use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

pub struct Report {
    pub shipped_green: Vec<String>,
    pub solution_red: Vec<(String, Status, String)>,
    pub total: usize,
}

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

    let exercises: Vec<_> = content
        .exercises
        .values()
        .filter(|e| only.is_none_or(|o| e.id.starts_with(o)))
        .collect();
    if exercises.is_empty() {
        bail!("no exercises match {only:?}");
    }

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
    // Project solutions may need more than one file (lib.rs + main.rs):
    // overlay solutions/<name>/src onto projects/<name>/src when present.
    let solutions_dir = root.join("solutions");
    if solutions_dir.exists() {
        for entry in std::fs::read_dir(&solutions_dir)? {
            let entry = entry?;
            let src = entry.path().join("src");
            let project = scratch.join("projects").join(entry.file_name());
            if src.is_dir() && project.is_dir() {
                copy_tree(&src, &project.join("src"))?;
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
            let detail = if outcome.stderr.trim().is_empty() {
                outcome.stdout
            } else {
                outcome.stderr
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
            solution_red.push((ex.id.clone(), outcome.status, format!("{detail}\n{checks}")));
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
