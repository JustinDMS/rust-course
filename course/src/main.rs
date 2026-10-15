//! The course server: serves the web app, the parsed content, the learner's
//! progress, and runs `cargo test` for exercises on request.
//!
//! ```text
//! cargo run -p course                 # serve on http://127.0.0.1:7878 and open the browser
//! cargo run -p course -- serve --port 9000 --no-open
//! cargo run -p course -- verify       # every exercise red as shipped, green with its solution
//! cargo run -p course -- verify ch04  # only one chapter
//! cargo run -p course -- list         # chapters, cards and exercises
//! ```

mod content;
mod playground;
mod progress;
mod runner;
mod verify;

use anyhow::{Context, Result};
use axum::{
    Json, Router,
    extract::{Path as AxumPath, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use content::Content;
use progress::Progress;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::services::{ServeDir, ServeFile};

struct AppState {
    root: PathBuf,
    progress: Mutex<Progress>,
    /// Only one cargo invocation at a time.
    cargo_lock: Mutex<()>,
    http: reqwest::Client,
}

type Shared = Arc<AppState>;

/// Course root: the directory holding `content/`, `web/`, `exercises/`.
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
        Some("verify") => {
            let content = Content::load(&root)?;
            let report = verify::verify(&content, args.get(1).map(String::as_str)).await?;
            println!();
            println!("{} exercises checked", report.total);
            for id in &report.shipped_green {
                println!("  !! {id} already passes as shipped");
            }
            for (id, status, detail) in &report.solution_red {
                println!("  !! {id} solution does not pass ({status:?}):");
                for line in detail.lines() {
                    println!("       {line}");
                }
            }
            if report.shipped_green.is_empty() && report.solution_red.is_empty() {
                println!("all good");
                Ok(())
            } else {
                std::process::exit(1);
            }
        }
        Some("list") => {
            let content = Content::load(&root)?;
            for ch in &content.chapters {
                let n_ex = content.exercises.values().filter(|e| e.chapter_id() == ch.id).count();
                println!("{:>2}. {:<40} {:>3} cards {:>2} exercises", ch.number, ch.title, ch.cards.len(), n_ex);
            }
            let course = content.course();
            println!("total XP available: {}", course.total_xp);
            Ok(())
        }
        Some("serve") | None => {
            let mut port: u16 = 7878;
            let mut open = true;
            let mut it = args.iter().skip(1);
            while let Some(a) = it.next() {
                match a.as_str() {
                    "--port" => port = it.next().context("--port needs a value")?.parse()?,
                    "--no-open" => open = false,
                    other => anyhow::bail!("unknown option {other}"),
                }
            }
            serve(root, port, open).await
        }
        Some(other) => anyhow::bail!("unknown command {other}; try serve, verify or list"),
    }
}

async fn serve(root: PathBuf, port: u16, open: bool) -> Result<()> {
    // Fail fast on content errors so they show up in the terminal.
    Content::load(&root).context("loading course content")?;
    let progress = Progress::load(&root.join("progress.json"))?;

    let state: Shared = Arc::new(AppState {
        root: root.clone(),
        progress: Mutex::new(progress),
        cargo_lock: Mutex::new(()),
        http: reqwest::Client::new(),
    });

    let web = root.join("web");
    let app = Router::new()
        .route("/api/course", get(api_course))
        .route("/api/progress", get(api_progress))
        .route("/api/progress/reset", post(api_progress_reset))
        .route("/api/cards/{id}/answer", post(api_card_answer))
        .route("/api/exercises/{ch}/{ex}/run", post(api_exercise_run))
        .route("/api/exercises/{ch}/{ex}/source", get(api_exercise_source))
        .route("/api/exercises/{ch}/{ex}/hint/{n}", get(api_exercise_hint))
        .route("/api/exercises/{ch}/{ex}/solution", get(api_exercise_solution))
        .route("/api/exercises/{ch}/{ex}/open", post(api_exercise_open))
        .route("/api/playground", post(api_playground))
        .fallback_service(ServeDir::new(&web).fallback(ServeFile::new(web.join("index.html"))))
        .with_state(state);

    let addr = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("binding {addr} (is another course server running?)"))?;
    let url = format!("http://{addr}");
    println!("Rust course is running at {url}");
    println!("Course root: {}", root.display());
    println!("Press Ctrl+C to stop.");
    if open {
        open_browser(&url);
    }
    axum::serve(listener, app).await?;
    Ok(())
}

fn open_browser(url: &str) {
    let result = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").args(["/C", "start", "", url]).spawn()
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("open").arg(url).spawn()
    } else {
        std::process::Command::new("xdg-open").arg(url).spawn()
    };
    if let Err(e) = result {
        eprintln!("could not open the browser ({e}); open {url} yourself");
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

struct AppError(StatusCode, anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({ "error": format!("{:#}", self.1) }));
        (self.0, body).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError(StatusCode::INTERNAL_SERVER_ERROR, e)
    }
}

fn not_found(e: anyhow::Error) -> AppError {
    AppError(StatusCode::NOT_FOUND, e)
}

type ApiResult<T> = std::result::Result<Json<T>, AppError>;

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

fn load(state: &AppState) -> Result<Content> {
    Content::load(&state.root).context("loading course content")
}

fn exercise_id(ch: &str, ex: &str) -> String {
    format!("{ch}/{ex}")
}

async fn save_progress(state: &AppState, progress: &Progress) -> Result<()> {
    progress.save(&state.root.join("progress.json"))
}

async fn api_course(State(state): State<Shared>) -> ApiResult<content::Course> {
    let content = load(&state)?;
    Ok(Json(content.course()))
}

async fn api_progress(State(state): State<Shared>) -> ApiResult<Progress> {
    Ok(Json(state.progress.lock().await.clone()))
}

async fn api_progress_reset(State(state): State<Shared>) -> ApiResult<Progress> {
    let mut progress = state.progress.lock().await;
    *progress = Progress::default();
    save_progress(&state, &progress).await?;
    Ok(Json(progress.clone()))
}

#[derive(Deserialize)]
struct AnswerBody {
    correct: bool,
}

#[derive(Serialize)]
struct AnswerReply {
    award: progress::Award,
    card: progress::CardProgress,
}

async fn api_card_answer(
    State(state): State<Shared>,
    AxumPath(id): AxumPath<String>,
    Json(body): Json<AnswerBody>,
) -> ApiResult<AnswerReply> {
    let content = load(&state)?;
    let xp = content
        .card_xp(&id)
        .ok_or_else(|| not_found(anyhow::anyhow!("unknown card {id}")))?;
    let mut progress = state.progress.lock().await;
    let award = progress.answer_card(&id, xp, body.correct, &content.index.levels);
    save_progress(&state, &progress).await?;
    let card = progress.cards.get(&id).cloned().unwrap_or_default();
    Ok(Json(AnswerReply { award, card }))
}

#[derive(Serialize)]
struct RunReply {
    outcome: runner::RunOutcome,
    award: progress::Award,
    exercise: progress::ExerciseProgress,
}

async fn api_exercise_run(
    State(state): State<Shared>,
    AxumPath((ch, ex)): AxumPath<(String, String)>,
) -> ApiResult<RunReply> {
    let content = load(&state)?;
    let id = exercise_id(&ch, &ex);
    let exercise = content.exercise(&id).map_err(not_found)?.clone();

    let outcome = {
        let _guard = state.cargo_lock.lock().await;
        runner::run_exercise(&state.root, &exercise, None).await?
    };

    let mut progress = state.progress.lock().await;
    let status = serde_json::to_value(outcome.status)
        .ok()
        .and_then(|v| v.as_str().map(str::to_owned))
        .unwrap_or_default();
    let award = progress.record_run(&id, exercise.xp, outcome.passed(), &status, &content.index.levels);
    save_progress(&state, &progress).await?;
    let ex_progress = progress.exercises.get(&id).cloned().unwrap_or_default();
    Ok(Json(RunReply {
        outcome,
        award,
        exercise: ex_progress,
    }))
}

#[derive(Serialize)]
struct SourceReply {
    path: String,
    source: String,
    /// Milliseconds since the Unix epoch; the frontend polls this for auto-run.
    modified_ms: u128,
}

async fn api_exercise_source(
    State(state): State<Shared>,
    AxumPath((ch, ex)): AxumPath<(String, String)>,
) -> ApiResult<SourceReply> {
    let content = load(&state)?;
    let exercise = content.exercise(&exercise_id(&ch, &ex)).map_err(not_found)?;
    let path = exercise.file_path(&state.root);
    let source = tokio::fs::read_to_string(&path)
        .await
        .with_context(|| format!("reading {}", path.display()))?;
    let modified_ms = tokio::fs::metadata(&path)
        .await
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_millis())
        .unwrap_or(0);
    Ok(Json(SourceReply {
        path: path.to_string_lossy().into_owned(),
        source,
        modified_ms,
    }))
}

#[derive(Serialize)]
struct HintReply {
    n: u32,
    total: usize,
    hint: String,
}

async fn api_exercise_hint(
    State(state): State<Shared>,
    AxumPath((ch, ex, n)): AxumPath<(String, String, u32)>,
) -> ApiResult<HintReply> {
    let content = load(&state)?;
    let id = exercise_id(&ch, &ex);
    let exercise = content.exercise(&id).map_err(not_found)?;
    let hint = exercise
        .hints
        .get(n.saturating_sub(1) as usize)
        .cloned()
        .ok_or_else(|| not_found(anyhow::anyhow!("{id} has no hint {n}")))?;
    let mut progress = state.progress.lock().await;
    progress.reveal_hint(&id, n);
    save_progress(&state, &progress).await?;
    Ok(Json(HintReply {
        n,
        total: exercise.hints.len(),
        hint,
    }))
}

#[derive(Serialize)]
struct SolutionReply {
    source: String,
}

async fn api_exercise_solution(
    State(state): State<Shared>,
    AxumPath((ch, ex)): AxumPath<(String, String)>,
) -> ApiResult<SolutionReply> {
    let content = load(&state)?;
    let id = exercise_id(&ch, &ex);
    let exercise = content.exercise(&id).map_err(not_found)?;
    let path = exercise.solution_path(&state.root);
    let source = tokio::fs::read_to_string(&path)
        .await
        .with_context(|| format!("reading {}", path.display()))?;
    let mut progress = state.progress.lock().await;
    progress.reveal_solution(&id);
    save_progress(&state, &progress).await?;
    Ok(Json(SolutionReply { source }))
}

async fn api_exercise_open(
    State(state): State<Shared>,
    AxumPath((ch, ex)): AxumPath<(String, String)>,
) -> ApiResult<serde_json::Value> {
    let content = load(&state)?;
    let exercise = content.exercise(&exercise_id(&ch, &ex)).map_err(not_found)?;
    let path = exercise.file_path(&state.root);
    let path_str = path.to_string_lossy().into_owned();
    let result = match std::env::var("COURSE_EDITOR") {
        Ok(editor) if !editor.trim().is_empty() => {
            let mut parts = editor.split_whitespace();
            let program = parts.next().unwrap_or_default().to_string();
            std::process::Command::new(program)
                .args(parts)
                .arg(&path)
                .spawn()
        }
        _ if cfg!(target_os = "windows") => std::process::Command::new("cmd")
            .args(["/C", "start", "", &path_str])
            .spawn(),
        _ if cfg!(target_os = "macos") => std::process::Command::new("open").arg(&path).spawn(),
        _ => std::process::Command::new("xdg-open").arg(&path).spawn(),
    };
    result.with_context(|| format!("opening {path_str}"))?;
    Ok(Json(serde_json::json!({ "opened": path_str })))
}

#[derive(Deserialize)]
struct PlaygroundBody {
    code: String,
    #[serde(default)]
    tests: bool,
}

#[derive(Serialize)]
struct PlaygroundReply {
    result: playground::ExecuteResponse,
    share_url: String,
}

async fn api_playground(
    State(state): State<Shared>,
    Json(body): Json<PlaygroundBody>,
) -> ApiResult<PlaygroundReply> {
    let result = playground::execute(&state.http, &body.code, body.tests)
        .await
        .map_err(|e| AppError(StatusCode::BAD_GATEWAY, e))?;
    Ok(Json(PlaygroundReply {
        result,
        share_url: playground::share_url(&body.code),
    }))
}
