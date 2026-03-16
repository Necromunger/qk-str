use qk_common::{exit_err, init, out, resolve_context};

fn main() {
    let args = init("fetch <url> — HTTP GET, body to stdout");
    let (url, _) = resolve_context(&args, 1).unwrap_or_else(|| exit_err("missing <url>"));

    match ureq::get(&url).call() {
        Ok(response) => match response.into_string() {
            Ok(body) => out(&body),
            Err(_) => exit_err("failed to read response body"),
        },
        Err(e) => exit_err(&format!("request failed: {e}")),
    }
}
