//! Proxy to the Rust Playground so lesson code cards can run in the browser.
//! Proxied (rather than called from the page) so the frontend never depends
//! on the Playground's CORS policy.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const EXECUTE_URL: &str = "https://play.rust-lang.org/execute";

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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteResponse {
    pub success: bool,
    #[serde(default)]
    pub exit_detail: String,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
}

pub async fn execute(client: &reqwest::Client, code: &str, tests: bool) -> Result<ExecuteResponse> {
    let body = ExecuteRequest {
        channel: "stable",
        mode: "debug",
        edition: "2024",
        crate_type: if tests { "lib" } else { "bin" },
        tests,
        code,
        backtrace: false,
    };
    let response = client
        .post(EXECUTE_URL)
        .timeout(Duration::from_secs(60))
        .json(&body)
        .send()
        .await
        .context("reaching play.rust-lang.org (are you online?)")?;
    let status = response.status();
    if !status.is_success() {
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("playground returned HTTP {status}: {}", text.chars().take(300).collect::<String>());
    }
    response
        .json::<ExecuteResponse>()
        .await
        .context("decoding the playground's reply")
}

/// A link that opens the code in the Playground's own editor.
pub fn share_url(code: &str) -> String {
    let mut encoded = String::new();
    for byte in code.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char)
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    format!("https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code={encoded}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn share_url_percent_encodes() {
        let url = share_url("fn main() {}");
        assert!(url.ends_with("code=fn%20main%28%29%20%7B%7D"));
    }
}
