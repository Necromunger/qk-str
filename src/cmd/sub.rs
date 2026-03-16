use crate::common::{exit_err, out, resolve_context};

pub const USAGE: &str = "qk sub <string> <offset> <length> — substring extraction";

pub fn run(args: Vec<String>) {
    let (s, off) = resolve_context(&args, 3).unwrap_or_else(|| exit_err("missing argument"));
    let offset: usize = args
        .get(off)
        .unwrap_or_else(|| exit_err("missing <offset>"))
        .parse()
        .unwrap_or_else(|_| exit_err("invalid offset"));
    let length: usize = args
        .get(off + 1)
        .unwrap_or_else(|| exit_err("missing <length>"))
        .parse()
        .unwrap_or_else(|_| exit_err("invalid length"));

    out(&substring(&s, offset, length));
}

fn substring(s: &str, offset: usize, length: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if offset > chars.len() {
        return String::new();
    }
    let end = (offset + length).min(chars.len());
    chars[offset..end].iter().collect()
}

#[cfg(test)]
mod tests {
    use super::substring;

    #[test]
    fn basic() {
        assert_eq!(substring("hello world", 6, 5), "world");
    }

    #[test]
    fn from_start() {
        assert_eq!(substring("hello", 0, 3), "hel");
    }

    #[test]
    fn beyond_length() {
        assert_eq!(substring("hi", 0, 100), "hi");
    }

    #[test]
    fn unicode() {
        assert_eq!(substring("café", 0, 4), "café");
    }

    #[test]
    fn offset_beyond() {
        assert_eq!(substring("hi", 10, 5), "");
    }
}
