use crate::common::{exit_err, out, resolve_context};

pub const USAGE: &str = "qk ascii <string> — strip non-ASCII characters";

pub fn run(args: Vec<String>) {
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|| exit_err("missing argument"));
    out(&strip_non_ascii(&s));
}

fn strip_non_ascii(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii()).collect()
}

#[cfg(test)]
mod tests {
    use super::strip_non_ascii;

    #[test]
    fn mixed() {
        assert_eq!(strip_non_ascii("café résumé"), "caf rsum");
    }

    #[test]
    fn all_ascii() {
        assert_eq!(strip_non_ascii("hello world"), "hello world");
    }

    #[test]
    fn all_unicode() {
        assert_eq!(strip_non_ascii("日本語"), "");
    }

    #[test]
    fn empty() {
        assert_eq!(strip_non_ascii(""), "");
    }

    #[test]
    fn preserves_punctuation() {
        assert_eq!(strip_non_ascii("hello! @#$ 世界"), "hello! @#$ ");
    }
}
