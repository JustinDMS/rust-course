//! Parses the course content: `content/index.toml`, `content/exercises.toml`
//! and one `content/chNN.md` lesson file per chapter.
//!
//! A lesson file is a sequence of cards separated by header lines:
//!
//! ```text
//! --- card: read id=c01 xp=5 title="What ownership is"
//! markdown body...
//! --- card: predict id=c02 xp=15 title="Does it compile?"
//! ```rust
//! fn main() { ... }
//! ```
//! ? What happens when this runs?
//! - [ ] It prints 5
//! - [x] It does not compile: use of moved value
//! > Explanation shown after answering.
//! --- card: quiz id=c03 xp=10 multi
//! ? Which of these are true?
//! - [x] ...
//! - [ ] ...
//! > explanation
//! --- card: quiz id=c04 xp=10 fill
//! ? Which keyword makes a binding changeable?
//! = mut
//! > explanation
//! --- card: exercise ref=ch04/ex01
//! ```

use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// index.toml
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct Index {
    pub title: String,
    pub levels: Vec<Level>,
    pub chapters: Vec<ChapterMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub xp: u32,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChapterMeta {
    pub id: String,
    pub number: u32,
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default)]
    pub book: String,
}

// ---------------------------------------------------------------------------
// exercises.toml
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
struct ExercisesFile {
    exercise: Vec<Exercise>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    /// `ch04/ex02`
    pub id: String,
    /// Workspace package name, e.g. `ch04_ownership`.
    #[serde(rename = "crate")]
    pub crate_name: String,
    /// `[[test]]` target name, e.g. `ex02_moves`.
    pub target: String,
    pub title: String,
    /// fix | implement | write | project
    pub kind: String,
    pub xp: u32,
    /// Markdown shown in the exercise panel.
    #[serde(default)]
    pub description: String,
    /// Path of the file to edit, relative to the course root. Defaults to
    /// `exercises/<crate>/<target>.rs`.
    #[serde(default)]
    pub file: String,
    /// Path of the solution file, relative to the course root. Defaults to
    /// `solutions/<crate>/<target>.rs`.
    #[serde(default)]
    pub solution: String,
    #[serde(default)]
    pub hints: Vec<String>,
    #[serde(default)]
    pub requires: Requires,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Requires {
    /// The edited file must contain at least this many `#[test]` attributes.
    #[serde(default)]
    pub min_tests: u32,
    /// The edited file must contain each of these substrings.
    #[serde(default)]
    pub must_contain: Vec<String>,
    /// The edited file must not contain any of these substrings.
    #[serde(default)]
    pub must_not_contain: Vec<String>,
}

impl Exercise {
    pub fn chapter_id(&self) -> &str {
        self.id.split('/').next().unwrap_or("")
    }

    pub fn file_path(&self, root: &Path) -> PathBuf {
        if self.file.is_empty() {
            root.join("exercises")
                .join(&self.crate_name)
                .join(format!("{}.rs", self.target))
        } else {
            root.join(&self.file)
        }
    }

    pub fn solution_path(&self, root: &Path) -> PathBuf {
        if self.solution.is_empty() {
            root.join("solutions")
                .join(&self.crate_name)
                .join(format!("{}.rs", self.target))
        } else {
            root.join(&self.solution)
        }
    }

    /// Project stages edit the project's `lib.rs`; their tests live apart,
    /// in `projects/<crate>/stages/<target>.rs`.
    pub fn is_project(&self) -> bool {
        self.kind == "project"
    }

    /// Where the tests are when they are not in the edited file itself.
    pub fn tests_path(&self, root: &Path) -> Option<PathBuf> {
        self.is_project().then(|| {
            root.join("projects")
                .join(&self.crate_name)
                .join("stages")
                .join(format!("{}.rs", self.target))
        })
    }

    /// The editor buffer this exercise works on. All stages of one project
    /// share the project's source, so they share a key.
    pub fn source_key(&self) -> String {
        if self.is_project() {
            self.crate_name.clone()
        } else {
            self.id.clone()
        }
    }

    /// The edited file's path relative to the course root, with `/`.
    pub fn display_file(&self, root: &Path) -> String {
        self.file_path(root)
            .strip_prefix(root)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default()
    }

    /// Everything the static site needs for this exercise.
    pub fn export(&self, root: &Path) -> Result<ExportedExercise> {
        let assembled = crate::online::assemble(self, root)?;
        Ok(ExportedExercise {
            id: self.id.clone(),
            chapter: self.chapter_id().to_string(),
            title: self.title.clone(),
            kind: self.kind.clone(),
            xp: self.xp,
            description: self.description.clone(),
            file: self.display_file(root),
            source_key: self.source_key(),
            starter: assembled.starter,
            tests: assembled.tests,
            solution: assembled.solution,
            hints: self.hints.clone(),
            requires: self.requires.clone(),
        })
    }
}

/// An exercise as `docs/data/course.js` carries it. Hints and the solution
/// are included; the site reveals them when the learner asks.
#[derive(Debug, Clone, Serialize)]
pub struct ExportedExercise {
    pub id: String,
    pub chapter: String,
    pub title: String,
    pub kind: String,
    pub xp: u32,
    pub description: String,
    /// Shown as the "file" being edited, e.g. `exercises/ch04_ownership/ex03_borrow_rules.rs`.
    pub file: String,
    /// Project stages share one editor buffer; everything else has its own.
    pub source_key: String,
    /// What the editor starts with.
    pub starter: String,
    /// Appended to the buffer before running, when the tests are separate.
    pub tests: Option<String>,
    pub solution: String,
    pub hints: Vec<String>,
    pub requires: Requires,
}

// ---------------------------------------------------------------------------
// Lesson cards
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Option_ {
    pub text: String,
    pub correct: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Card {
    Read {
        id: String,
        xp: u32,
        title: String,
        body: String,
    },
    Code {
        id: String,
        xp: u32,
        title: String,
        code: String,
        /// Markdown shown under the code.
        body: String,
    },
    Predict {
        id: String,
        xp: u32,
        title: String,
        code: String,
        question: String,
        options: Vec<Option_>,
        explanation: String,
    },
    Quiz {
        id: String,
        xp: u32,
        title: String,
        /// Markdown shown above the question (optional context, may hold code).
        body: String,
        question: String,
        multi: bool,
        options: Vec<Option_>,
        /// For `fill` quizzes: accepted answers (case-insensitive).
        answers: Vec<String>,
        explanation: String,
    },
    Exercise {
        id: String,
        #[serde(rename = "ref")]
        exercise: String,
    },
}

impl Card {
    pub fn id(&self) -> &str {
        match self {
            Card::Read { id, .. }
            | Card::Code { id, .. }
            | Card::Predict { id, .. }
            | Card::Quiz { id, .. }
            | Card::Exercise { id, .. } => id,
        }
    }

    pub fn xp(&self) -> u32 {
        match self {
            Card::Read { xp, .. }
            | Card::Code { xp, .. }
            | Card::Predict { xp, .. }
            | Card::Quiz { xp, .. } => *xp,
            Card::Exercise { .. } => 0,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Chapter {
    pub id: String,
    pub number: u32,
    pub title: String,
    pub subtitle: String,
    pub book: String,
    pub cards: Vec<Card>,
}

/// Everything the static site needs; serialised into `docs/data/course.js`.
#[derive(Debug, Clone, Serialize)]
pub struct Course {
    pub title: String,
    pub levels: Vec<Level>,
    pub chapters: Vec<Chapter>,
    pub exercises: Vec<ExportedExercise>,
    /// Total XP available, for the level bar.
    pub total_xp: u32,
}

/// The full, server-side view (keeps hints and solution paths).
#[derive(Debug, Clone)]
pub struct Content {
    pub root: PathBuf,
    pub index: Index,
    pub chapters: Vec<Chapter>,
    pub exercises: BTreeMap<String, Exercise>,
}

impl Content {
    pub fn load(root: &Path) -> Result<Content> {
        let dir = root.join("content");
        let index_text = std::fs::read_to_string(dir.join("index.toml"))
            .with_context(|| format!("reading {}", dir.join("index.toml").display()))?;
        let index: Index = toml::from_str(&index_text).context("parsing content/index.toml")?;

        let ex_text = std::fs::read_to_string(dir.join("exercises.toml"))
            .context("reading content/exercises.toml")?;
        let ex_file: ExercisesFile =
            toml::from_str(&ex_text).context("parsing content/exercises.toml")?;
        let mut exercises = BTreeMap::new();
        for ex in ex_file.exercise {
            if exercises.insert(ex.id.clone(), ex.clone()).is_some() {
                bail!("duplicate exercise id {}", ex.id);
            }
        }

        let mut chapters = Vec::new();
        for meta in &index.chapters {
            let path = dir.join(format!("{}.md", meta.id));
            let cards = if path.exists() {
                let text = std::fs::read_to_string(&path)
                    .with_context(|| format!("reading {}", path.display()))?;
                parse_cards(&meta.id, &text)
                    .with_context(|| format!("parsing {}", path.display()))?
            } else {
                Vec::new()
            };
            for card in &cards {
                if let Card::Exercise { exercise, .. } = card
                    && !exercises.contains_key(exercise)
                {
                    bail!("{}: card references unknown exercise {exercise}", meta.id);
                }
            }
            chapters.push(Chapter {
                id: meta.id.clone(),
                number: meta.number,
                title: meta.title.clone(),
                subtitle: meta.subtitle.clone(),
                book: meta.book.clone(),
                cards,
            });
        }

        Ok(Content {
            root: root.to_path_buf(),
            index,
            chapters,
            exercises,
        })
    }

    pub fn course(&self) -> Result<Course> {
        let exercises = self
            .exercises
            .values()
            .map(|e| e.export(&self.root))
            .collect::<Result<Vec<_>>>()?;
        let total_xp = self
            .chapters
            .iter()
            .flat_map(|c| c.cards.iter().map(Card::xp))
            .sum::<u32>()
            + self.exercises.values().map(|e| e.xp).sum::<u32>();
        Ok(Course {
            title: self.index.title.clone(),
            levels: self.index.levels.clone(),
            chapters: self.chapters.clone(),
            exercises,
            total_xp,
        })
    }
}

// ---------------------------------------------------------------------------
// Card parser
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Header {
    kind: String,
    attrs: BTreeMap<String, String>,
    flags: Vec<String>,
}

fn parse_header(line: &str) -> Result<Header> {
    let rest = line
        .trim()
        .strip_prefix("--- card:")
        .ok_or_else(|| anyhow!("not a card header: {line}"))?
        .trim();
    let mut tokens = tokenize(rest);
    if tokens.is_empty() {
        bail!("card header without a type: {line}");
    }
    let kind = tokens.remove(0);
    let mut header = Header {
        kind,
        ..Default::default()
    };
    for tok in tokens {
        if let Some((k, v)) = tok.split_once('=') {
            header.attrs.insert(k.to_string(), v.to_string());
        } else {
            header.flags.push(tok);
        }
    }
    Ok(header)
}

/// Splits on whitespace, honouring double quotes: `title="Does it compile?"`.
fn tokenize(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quote = false;
    for ch in s.chars() {
        match ch {
            '"' => in_quote = !in_quote,
            c if c.is_whitespace() && !in_quote => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            c => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

pub fn parse_cards(chapter_id: &str, text: &str) -> Result<Vec<Card>> {
    let mut cards = Vec::new();
    let mut header: Option<Header> = None;
    let mut body: Vec<&str> = Vec::new();
    let mut seen_ids = std::collections::HashSet::new();

    let flush = |header: Option<Header>, body: &mut Vec<&str>, cards: &mut Vec<Card>| -> Result<()> {
        if let Some(h) = header {
            let card = build_card(chapter_id, h, &body.join("\n"))?;
            cards.push(card);
        }
        body.clear();
        Ok(())
    };

    for line in text.lines() {
        if line.trim_start().starts_with("--- card:") {
            flush(header.take(), &mut body, &mut cards)?;
            header = Some(parse_header(line)?);
        } else if header.is_some() {
            body.push(line);
        } else if !line.trim().is_empty() && !line.trim_start().starts_with('#') {
            // Text before the first card: allow a title line / comment only.
            bail!("content before the first card: {line:?}");
        }
    }
    flush(header.take(), &mut body, &mut cards)?;

    for card in &cards {
        if !seen_ids.insert(card.id().to_string()) {
            bail!("duplicate card id {}", card.id());
        }
    }
    Ok(cards)
}

fn attr_u32(h: &Header, key: &str, default: u32) -> Result<u32> {
    match h.attrs.get(key) {
        Some(v) => v
            .parse()
            .with_context(|| format!("card attribute {key}={v} is not a number")),
        None => Ok(default),
    }
}

fn build_card(chapter_id: &str, h: Header, body: &str) -> Result<Card> {
    let title = h.attrs.get("title").cloned().unwrap_or_default();
    let make_id = |h: &Header| -> Result<String> {
        let raw = h
            .attrs
            .get("id")
            .ok_or_else(|| anyhow!("{} card is missing id=", h.kind))?;
        Ok(format!("{chapter_id}-{raw}"))
    };

    match h.kind.as_str() {
        "read" => Ok(Card::Read {
            id: make_id(&h)?,
            xp: attr_u32(&h, "xp", 5)?,
            title,
            body: body.trim().to_string(),
        }),
        "code" => {
            let parts = split_body(body);
            let code = parts
                .code
                .ok_or_else(|| anyhow!("code card {} has no ```rust block", title))?;
            Ok(Card::Code {
                id: make_id(&h)?,
                xp: attr_u32(&h, "xp", 5)?,
                title,
                code,
                body: parts.prose.trim().to_string(),
            })
        }
        "predict" => {
            let parts = split_body(body);
            let code = parts
                .code
                .ok_or_else(|| anyhow!("predict card {title:?} has no ```rust block"))?;
            if parts.options.is_empty() {
                bail!("predict card {title:?} has no options");
            }
            if !parts.options.iter().any(|o| o.correct) {
                bail!("predict card {title:?} has no correct option");
            }
            Ok(Card::Predict {
                id: make_id(&h)?,
                xp: attr_u32(&h, "xp", 15)?,
                title,
                code,
                question: parts.question,
                options: parts.options,
                explanation: parts.explanation,
            })
        }
        "quiz" => {
            let parts = split_body(body);
            let fill = h.flags.iter().any(|f| f == "fill");
            let multi = h.flags.iter().any(|f| f == "multi");
            if fill {
                if parts.answers.is_empty() {
                    bail!("fill quiz {title:?} has no `= answer` line");
                }
            } else {
                if parts.options.is_empty() {
                    bail!("quiz {title:?} has no options");
                }
                let correct = parts.options.iter().filter(|o| o.correct).count();
                if correct == 0 {
                    bail!("quiz {title:?} has no correct option");
                }
                if correct > 1 && !multi {
                    bail!("quiz {title:?} has several correct options but no `multi` flag");
                }
            }
            let mut prose = parts.prose.trim().to_string();
            if let Some(code) = &parts.code {
                prose = format!("{prose}\n\n```rust\n{code}\n```").trim().to_string();
            }
            Ok(Card::Quiz {
                id: make_id(&h)?,
                xp: attr_u32(&h, "xp", 10)?,
                title,
                body: prose,
                question: parts.question,
                multi,
                options: parts.options,
                answers: parts.answers,
                explanation: parts.explanation,
            })
        }
        "exercise" => {
            let exercise = h
                .attrs
                .get("ref")
                .ok_or_else(|| anyhow!("exercise card is missing ref="))?
                .clone();
            let id = format!("{chapter_id}-{}", exercise.replace('/', "-"));
            Ok(Card::Exercise { id, exercise })
        }
        other => bail!("unknown card type {other:?}"),
    }
}

#[derive(Default)]
struct Parts {
    prose: String,
    code: Option<String>,
    question: String,
    options: Vec<Option_>,
    answers: Vec<String>,
    explanation: String,
}

/// Splits a card body into the first ```rust fenced block, prose, the `?`
/// question, `- [ ]` options, `=` answers and `>` explanation.
fn split_body(body: &str) -> Parts {
    let mut parts = Parts::default();
    let mut in_fence = false;
    let mut fence_lines: Vec<&str> = Vec::new();
    let mut prose: Vec<&str> = Vec::new();
    let mut question: Vec<&str> = Vec::new();
    let mut explanation: Vec<&str> = Vec::new();
    #[derive(PartialEq)]
    enum Mode {
        Prose,
        Question,
        Options,
        Explanation,
    }
    let mut mode = Mode::Prose;

    for line in body.lines() {
        let t = line.trim_start();
        if t.starts_with("```") {
            if in_fence {
                in_fence = false;
                if parts.code.is_none() {
                    parts.code = Some(fence_lines.join("\n"));
                } else {
                    // Later fences stay in the prose as markdown.
                    prose.push("```rust");
                    prose.extend(fence_lines.iter().copied());
                    prose.push("```");
                }
                fence_lines.clear();
            } else {
                in_fence = true;
            }
            continue;
        }
        if in_fence {
            fence_lines.push(line);
            continue;
        }
        if let Some(q) = t.strip_prefix("? ") {
            question.clear();
            question.push(q);
            mode = Mode::Question;
        } else if let Some(opt) = t.strip_prefix("- [ ] ") {
            parts.options.push(Option_ {
                text: opt.trim().to_string(),
                correct: false,
            });
            mode = Mode::Options;
        } else if let Some(opt) = t.strip_prefix("- [x] ").or_else(|| t.strip_prefix("- [X] ")) {
            parts.options.push(Option_ {
                text: opt.trim().to_string(),
                correct: true,
            });
            mode = Mode::Options;
        } else if let Some(ans) = t.strip_prefix("= ") {
            parts
                .answers
                .extend(ans.split('|').map(|a| a.trim().to_string()).filter(|a| !a.is_empty()));
            mode = Mode::Options;
        } else if t.starts_with('>') {
            explanation.push(t.trim_start_matches('>').trim_start());
            mode = Mode::Explanation;
        } else {
            match mode {
                Mode::Prose => prose.push(line),
                Mode::Question => {
                    if t.is_empty() {
                        mode = Mode::Options;
                    } else {
                        question.push(t);
                    }
                }
                Mode::Options => {
                    if !t.is_empty() {
                        // Continuation of the last option text.
                        if let Some(last) = parts.options.last_mut() {
                            last.text.push(' ');
                            last.text.push_str(t);
                        }
                    }
                }
                Mode::Explanation => explanation.push(t),
            }
        }
    }
    parts.prose = prose.join("\n");
    parts.question = question.join(" ").trim().to_string();
    parts.explanation = explanation.join("\n").trim().to_string();
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"# Chapter 4
--- card: read id=c01 xp=5 title="Moves"
Ownership moves **values**.

Second paragraph.
--- card: code id=c02 title="Try it"
```rust
fn main() { println!("hi"); }
```
Run this and look at the output.
--- card: predict id=c03 xp=20 title="Does it compile?"
```rust
let s = String::from("a");
let t = s;
println!("{s}");
```
? What happens?
- [ ] Prints `a`
- [x] Compile error E0382
> `s` was moved into `t`.
> Second line.
--- card: quiz id=c04 multi
? Which types are Copy?
- [x] i32
- [ ] String
- [x] bool
> Scalars are Copy.
--- card: quiz id=c05 fill xp=10
? Which keyword allows mutation?
= mut | mutable
> `mut`.
--- card: exercise ref=ch04/ex01
"#;

    #[test]
    fn parses_every_card_type() {
        let cards = parse_cards("ch04", SAMPLE).unwrap();
        assert_eq!(cards.len(), 6);
        match &cards[0] {
            Card::Read { id, xp, title, body } => {
                assert_eq!(id, "ch04-c01");
                assert_eq!(*xp, 5);
                assert_eq!(title, "Moves");
                assert!(body.starts_with("Ownership moves"));
                assert!(body.ends_with("Second paragraph."));
            }
            other => panic!("{other:?}"),
        }
        match &cards[1] {
            Card::Code { code, body, xp, .. } => {
                assert_eq!(code, "fn main() { println!(\"hi\"); }");
                assert_eq!(body, "Run this and look at the output.");
                assert_eq!(*xp, 5);
            }
            other => panic!("{other:?}"),
        }
        match &cards[2] {
            Card::Predict {
                code,
                question,
                options,
                explanation,
                xp,
                ..
            } => {
                assert_eq!(code.lines().count(), 3);
                assert_eq!(question, "What happens?");
                assert_eq!(options.len(), 2);
                assert!(!options[0].correct);
                assert!(options[1].correct);
                assert_eq!(explanation, "`s` was moved into `t`.\nSecond line.");
                assert_eq!(*xp, 20);
            }
            other => panic!("{other:?}"),
        }
        match &cards[3] {
            Card::Quiz { multi, options, .. } => {
                assert!(multi);
                assert_eq!(options.iter().filter(|o| o.correct).count(), 2);
            }
            other => panic!("{other:?}"),
        }
        match &cards[4] {
            Card::Quiz { answers, options, .. } => {
                assert_eq!(answers, &["mut", "mutable"]);
                assert!(options.is_empty());
            }
            other => panic!("{other:?}"),
        }
        match &cards[5] {
            Card::Exercise { id, exercise } => {
                assert_eq!(id, "ch04-ch04-ex01");
                assert_eq!(exercise, "ch04/ex01");
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn rejects_quiz_without_correct_answer() {
        let text = "--- card: quiz id=c01\n? Q\n- [ ] a\n- [ ] b\n";
        assert!(parse_cards("ch01", text).is_err());
    }

    #[test]
    fn rejects_duplicate_ids() {
        let text = "--- card: read id=c01\nA\n--- card: read id=c01\nB\n";
        assert!(parse_cards("ch01", text).is_err());
    }

    #[test]
    fn tokenizer_honours_quotes() {
        assert_eq!(
            tokenize(r#"read id=c01 title="Does it compile?" multi"#),
            vec!["read", "id=c01", "title=Does it compile?", "multi"]
        );
    }
}
