use qk_common::{exit_err, init, out};
use std::io::Read;

fn main() {
    let args = init("join <delimiter> — join stdin lines with delimiter");
    let delim = args
        .first()
        .unwrap_or_else(|| exit_err("missing <delimiter>"));

    let mut buf = String::new();
    if std::io::stdin().read_to_string(&mut buf).is_err() {
        exit_err("cannot read stdin");
    }

    let lines: Vec<&str> = buf.lines().collect();
    out(&lines.join(delim.as_str()));
}

#[cfg(test)]
mod tests {
    fn join_lines(input: &str, delim: &str) -> String {
        let lines: Vec<&str> = input.lines().collect();
        lines.join(delim)
    }

    #[test]
    fn basic() {
        assert_eq!(join_lines("a\nb\nc", ","), "a,b,c");
    }

    #[test]
    fn single_line() {
        assert_eq!(join_lines("hello", ","), "hello");
    }

    #[test]
    fn empty_delimiter() {
        assert_eq!(join_lines("a\nb\nc", ""), "abc");
    }

    #[test]
    fn trailing_newline() {
        assert_eq!(join_lines("a\nb\nc\n", ","), "a,b,c");
    }
}
