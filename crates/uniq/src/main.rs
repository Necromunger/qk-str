use qk_common::{exit_err, init, out, resolve_context};
use std::collections::BTreeSet;

fn dedup_chars(s: &str) -> String {
    let mut seen = BTreeSet::new();
    let mut result = String::new();
    for c in s.chars() {
        if seen.insert(c) {
            result.push(c);
        }
    }
    result
}

fn main() {
    let args = init("uniq <string> — deduplicate characters (preserving order)");
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|| exit_err("missing argument"));
    out(&dedup_chars(&s));
}

#[cfg(test)]
mod tests {
    use super::dedup_chars;

    #[test]
    fn basic() {
        assert_eq!(dedup_chars("abcabc"), "abc");
    }

    #[test]
    fn no_dupes() {
        assert_eq!(dedup_chars("abc"), "abc");
    }

    #[test]
    fn empty() {
        assert_eq!(dedup_chars(""), "");
    }

    #[test]
    fn preserves_order() {
        assert_eq!(dedup_chars("banana"), "ban");
    }
}
