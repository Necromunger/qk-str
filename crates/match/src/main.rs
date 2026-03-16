use qk_common::{exit_err, init, out, resolve_context};
use regex::Regex;

fn main() {
    let args = init("match <string> <pattern> — first regex match (exit 1 if none)");
    let (s, off) = resolve_context(&args, 2).unwrap_or_else(|_| exit_err("missing argument"));
    let pattern = args.get(off).unwrap_or_else(|| exit_err("missing <pattern>"));
    let re = Regex::new(pattern).unwrap_or_else(|_| exit_err("invalid regex"));

    match re.find(&s) {
        Some(m) => out(m.as_str()),
        None => std::process::exit(1),
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    fn first_match(s: &str, pattern: &str) -> Option<String> {
        Regex::new(pattern).ok()?.find(s).map(|m| m.as_str().to_string())
    }

    #[test]
    fn found() { assert_eq!(first_match("hello123", r"\d+"), Some("123".to_string())); }

    #[test]
    fn not_found() { assert_eq!(first_match("hello", r"\d+"), None); }

    #[test]
    fn full_match() { assert_eq!(first_match("abc", r"[a-z]+"), Some("abc".to_string())); }

    #[test]
    fn invalid_regex() { assert_eq!(first_match("abc", r"["), None); }
}
