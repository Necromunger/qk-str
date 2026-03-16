use crate::common::{exit_err, out, resolve_context};

const TIMEOUT_SECS: u64 = 10;

pub const USAGE: &str = "qk fetch <url> — HTTP GET, body to stdout";

pub fn run(args: Vec<String>) {
    let (url, _) = resolve_context(&args, 1).unwrap_or_else(|| exit_err("missing <url>"));

    let agent = ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(TIMEOUT_SECS))
        .build();
    match agent.get(&url).call() {
        Ok(response) => match response.into_string() {
            Ok(body) => out(&body),
            Err(_) => exit_err("failed to read response body"),
        },
        Err(e) => exit_err(&format!("request failed: {e}")),
    }
}
