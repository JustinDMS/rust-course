//! The course build tool. The course itself is a static site in `docs/`
//! (open `docs/index.html` or publish the folder); this binary generates the
//! site's data file from `content/`, `exercises/` and `solutions/`, and
//! checks the exercises.
//!
//! ```text
//! cargo run -p course -- build              # write docs/data/course.js
//! cargo run -p course -- verify             # every exercise red as shipped, green with its solution (local cargo)
//! cargo run -p course -- verify --online    # the same, compiled on play.rust-lang.org exactly as the site does it
//! cargo run -p course -- verify ch04        # only one chapter (works with --online too)
//! cargo run -p course -- list               # chapters, cards and exercises
//! ```

mod content;
mod export;
mod online;
mod runner;
mod verify;

use anyhow::{Result, bail};
use content::Content;
use std::path::{Path, PathBuf};

const USAGE: &str = "usage: cargo run -p course -- <build | verify [--online] [chNN] | list>";

/// Course root: the directory holding `content/`, `docs/`, `exercises/`.
fn course_root() -> PathBuf {
    if let Ok(dir) = std::env::var("COURSE_ROOT") {
        return PathBuf::from(dir);
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("course crate lives inside the course root")
        .to_path_buf()
}

#[tokio::main]
async fn main() -> Result<()> {
    let root = course_root();
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("build") => {
            let content = Content::load(&root)?;
            let summary = export::build(&content, &root.join("docs"))?;
            println!(
                "wrote {} ({} KB): {} chapters, {} cards, {} exercises, {} XP",
                summary.path.display(),
                summary.bytes / 1024,
                summary.chapters,
                summary.cards,
                summary.exercises,
                summary.total_xp
            );
            Ok(())
        }
        Some("verify") => {
            let content = Content::load(&root)?;
            let mut online = false;
            let mut only: Option<&str> = None;
            for arg in &args[1..] {
                match arg.as_str() {
                    "--online" => online = true,
                    s if s.starts_with("ch") => only = Some(s),
                    other => bail!("unknown option {other}\n{USAGE}"),
                }
            }
            let report = if online {
                verify::verify_online(&content, only)?
            } else {
                verify::verify(&content, only).await?
            };
            report.print();
            if report.ok() {
                Ok(())
            } else {
                std::process::exit(1);
            }
        }
        Some("list") => {
            let content = Content::load(&root)?;
            for ch in &content.chapters {
                let n_ex = content.exercises.values().filter(|e| e.chapter_id() == ch.id).count();
                println!(
                    "{:>2}. {:<40} {:>3} cards {:>2} exercises",
                    ch.number,
                    ch.title,
                    ch.cards.len(),
                    n_ex
                );
            }
            let course = content.course()?;
            println!("total XP available: {}", course.total_xp);
            Ok(())
        }
        None | Some("help") | Some("--help") | Some("-h") => {
            println!("{USAGE}");
            Ok(())
        }
        Some(other) => bail!("unknown command {other}\n{USAGE}"),
    }
}
