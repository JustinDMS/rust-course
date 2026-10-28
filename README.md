# The Rust Programming Language, interactive

An interactive course built from *The Rust Programming Language* (chapters 1 to 13).
It is a static website: lessons with concise explanations, runnable code cards,
predict-the-output cards and quizzes, plus 62 exercises that you write in an editor
in the page and check with real `cargo test` runs on the
[Rust Playground](https://play.rust-lang.org). Nothing to install.

**Live site:** https://justindms.github.io/rust-course/

Progress (XP, answered cards, exercise history) and your code are saved in the
browser. *Export progress* in the sidebar writes them to a JSON file; *Import* on
another device loads it. A `progress.json` from the older, server-based version of
this course imports too.

## How exercises work

Every exercise is one Rust file: a doc comment that says what to do, the code, and
the `#[test]` functions that decide whether you are done. You edit it in the card,
press *Run tests* (or Ctrl+Enter), and the file is sent to the Playground as a
library crate with `tests: true`. The compiler's and test harness's output comes
back as is, with error codes linked to the Rust error index.

- Hints are free and revealed one at a time.
- The solution unlocks after two attempts and halves the exercise's XP.
- Some exercises have extra checks beyond the tests (for example "no `.clone()` in
  this file" or "at least four `#[test]` functions"); the app lists them after a run.
- The two projects (the guessing game, minigrep) are built in stages that all edit
  one shared file; each stage appends its own tests when you run it.

## Working on the course

The site in `docs/` is generated from the content in this repository. The Rust
toolchain is only needed to regenerate it.

```bash
cargo run -p course -- build             # content/ + exercises/ + solutions/ -> docs/data/course.js
cargo run -p course -- verify            # every exercise red as shipped, green with its solution (local cargo)
cargo run -p course -- verify --online   # the same through the Playground, exactly as the site runs them
cargo run -p course -- verify ch04       # one chapter (works with --online)
cargo run -p course -- list              # chapters, cards, exercises, XP
cargo test -p course                     # the build tool's own tests
```

To look at the site locally, serve `docs/` with any static server, for example:

```bash
python -m http.server 8008 -d docs
```

Opening `docs/index.html` straight from the file system also works, with a plain
textarea instead of the CodeMirror editor (browsers do not load module scripts
from `file://`).

### Publishing

The site is served by GitHub Pages from the `docs/` folder of the `main` branch
(repository settings, Pages, "Deploy from a branch", `main` and `/docs`). Commit
the regenerated `docs/data/course.js` together with content changes.

## Layout

```text
course/        the build tool: parses content/, assembles exercises, writes docs/data/course.js, runs verify
content/       index.toml (chapters, levels), exercises.toml, chNN.md lesson cards
docs/          the site: index.html, app.js, editor.js, style.css, md.js, hl.js, data/course.js (generated)
exercises/     one crate per chapter, one [[test]] target per exercise (the source of the starter code)
projects/      guessing_game, minigrep: src/lib.rs is the shared editor file, stages/*.rs the per-stage tests
solutions/     reference solutions, mirrored on exercises/ and projects/
```

## Content format

`content/chNN.md` is a sequence of cards separated by `--- card: <type> key=value ...`
lines: `read`, `code`, `predict`, `quiz` (with `multi` or `fill` flags) and
`exercise ref=chNN/exNN`. Quiz options are `- [ ]` / `- [x]` lines, the question is a
`? ` line, the explanation is a `> ` block. `content/exercises.toml` holds each
exercise's metadata, hints and extra checks.

After changing content, exercises or solutions: `verify` (and `verify --online` for
anything that touches what the Playground sees, such as crates or file access),
then `build`, then commit.
