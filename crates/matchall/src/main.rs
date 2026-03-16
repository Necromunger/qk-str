use qk_common::{exit_err, init, out, resolve_context};
use regex::Regex;

fn main() {
    let args = init("matchall <string> <pattern> — all regex matches, one per line");
    let (s, off) = resolve_context(&args, 2).unwrap_or_else(|_| exit_err("missing argument"));
    let pattern = args.get(off).unwrap_or_else(|| exit_err("missing <pattern>"));
    let re = Regex::new(pattern).unwrap_or_else(|_| exit_err("invalid regex"));

    let matches: Vec<&str> = re.find_iter(&s).map(|m| m.as_str()).collect();
    if matches.is_empty() {
        std::process::exit(1);
    }
    for m in matches {
        out(m);
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    fn all_matches(s: &str, pattern: &str) -> Vec<String> {
        Regex::new(pattern).unwrap().find_iter(s).map(|m| m.as_str().to_string()).collect()
    }

    #[test]
    fn multiple() { assert_eq!(all_matches("a1b2c3", r"\d"), vec!["1", "2", "3"]); }

    #[test]
    fn none() { assert!(all_matches("abc", r"\d").is_empty()); }

    #[test]
    fn single() { assert_eq!(all_matches("hello123", r"\d+"), vec!["123"]); }
}
