use crate::common::{exit_err, out, resolve_context};

pub const USAGE: &str = "qk rev <string> — reverse the string";

pub fn run(args: Vec<String>) {
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|| exit_err("missing argument"));
    out(&s.chars().rev().collect::<String>());
}

#[cfg(test)]
mod tests {
    fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }

    #[test]
    fn basic() {
        assert_eq!(reverse("hello"), "olleh");
    }

    #[test]
    fn palindrome() {
        assert_eq!(reverse("racecar"), "racecar");
    }

    #[test]
    fn empty() {
        assert_eq!(reverse(""), "");
    }

    #[test]
    fn unicode() {
        assert_eq!(reverse("café"), "éfac");
    }
}
