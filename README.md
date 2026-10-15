# The Rust Programming Language, interactive

An interactive course built from *The Rust Programming Language* (chapters 1 to 13).
Two halves:

- **A web app** with the lessons: concise explanations, runnable code cards (via the
  Rust Playground), predict-the-output cards and quizzes. Earns XP.
- **A Cargo workspace of exercises** that reinforce each chapter: fix-the-compile-error,
  implement-the-function, write-the-tests, and two staged projects (the guessing game
  and minigrep). Checked by `cargo test`, run from the web app with real compiler output.

## Start

```bash
cargo run -p course
```

That builds the server, opens `http://127.0.0.1:7878` in your browser, and serves the
course. Pick a chapter in the sidebar and work down the cards. Progress (XP, answered
cards, exercise history) is saved to `progress.json` in this folder.

Other commands:

```bash
cargo run -p course -- serve --port 9000 --no-open   # different port, no browser
cargo run -p course -- list                          # chapters, cards, exercises, XP
cargo run -p course -- verify                        # content CI (see below)
cargo run -p course -- verify ch04                   # just one chapter
```

Set `COURSE_EDITOR` (for example `code` or `code -g`) to control what the *Open file*
button launches; by default it uses the system's default program for `.rs` files.

## How exercises work

Every chapter has a crate under `exercises/`, and every exercise is one file in it,
compiled as its own test target:

```text
exercises/ch04_ownership/ex03_borrow_rules.rs
cargo test -p ch04_ownership --test ex03_borrow_rules
```

The file starts with a doc comment that says what to do, then the code, then the
`#[test]` functions that decide whether you are done. Edit the file in your editor,
press *Run tests* in the app (or turn on *Run on save*), read the compiler.

- Hints are free and revealed one at a time.
- The solution unlocks after two attempts and halves the exercise's XP.
- Some exercises have extra checks beyond the tests (for example "no `.clone()` in this
  file" or "at least four `#[test]` functions"); the app lists them after a run.

The projects live in `projects/guessing_game` and `projects/minigrep`. Their `main.rs`
files are complete and playable; the library code is yours, checked stage by stage.

## Layout

```text
course/        the server (axum): serves web/, parses content/, runs cargo test
content/       index.toml (chapters, levels), exercises.toml, chNN.md lesson cards
web/           the frontend (vanilla JS, no build step)
exercises/     one crate per chapter, one [[test]] target per exercise
projects/      guessing_game, minigrep
solutions/     reference solutions, mirrored on exercises/ and projects/
```

## Content format

`content/chNN.md` is a sequence of cards separated by `--- card: <type> key=value ...`
lines: `read`, `code`, `predict`, `quiz` (with `multi` or `fill` flags) and
`exercise ref=chNN/exNN`. Quiz options are `- [ ]` / `- [x]` lines, the question is a
`? ` line, the explanation is a `> ` block. `content/exercises.toml` holds each
exercise's metadata, hints and extra checks. The server re-reads content on every
request, so edits show up on refresh without a rebuild.

`cargo run -p course -- verify` copies the exercise crates to a scratch workspace and
checks that every exercise fails as shipped and passes with its solution overlaid. Run
it after changing any exercise or solution.
