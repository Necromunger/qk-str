use crate::common::{exit_err, out};
use std::io::Read;

pub const USAGE: &str = "qk lines <file-or-stdin> — count lines";

pub fn run(args: Vec<String>) {
    if let Some(path) = args.first() {
        match std::fs::read_to_string(path) {
            Ok(content) => {
                out(&count_lines(&content).to_string());
                return;
            }
            Err(_) => exit_err("cannot read file"),
        }
    }

    let mut buf = String::new();
    if std::io::stdin().read_to_string(&mut buf).is_err() {
        exit_err("cannot read stdin");
    }
    out(&count_lines(&buf).to_string());
}

fn count_lines(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }
    s.lines().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(count_lines(""), 0);
    }

    #[test]
    fn one_line() {
        assert_eq!(count_lines("hello"), 1);
    }

    #[test]
    fn multiple() {
        assert_eq!(count_lines("a\nb\nc"), 3);
    }

    #[test]
    fn trailing_newline() {
        assert_eq!(count_lines("a\nb\n"), 2);
    }
}
