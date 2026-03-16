use crate::common::{exit_err, out, resolve_context};

pub const USAGE: &str = "qk chars <string> — one character per line";

pub fn run(args: Vec<String>) {
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|| exit_err("missing argument"));
    for c in s.chars() {
        out(&c.to_string());
    }
}

#[cfg(test)]
mod tests {
    fn chars_to_lines(s: &str) -> String {
        s.chars()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn basic() {
        assert_eq!(chars_to_lines("hello"), "h\ne\nl\nl\no");
    }

    #[test]
    fn empty() {
        assert_eq!(chars_to_lines(""), "");
    }

    #[test]
    fn unicode() {
        assert_eq!(chars_to_lines("ab"), "a\nb");
    }

    #[test]
    fn single_char() {
        assert_eq!(chars_to_lines("x"), "x");
    }
}
