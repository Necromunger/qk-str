use crate::common::{exit_err, out, resolve_context};

pub const USAGE: &str = "qk urlencode <string> — percent-encode for URLs";

pub fn run(args: Vec<String>) {
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|| exit_err("missing argument"));
    out(&encode(&s));
}

fn encode(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(b as char);
            }
            _ => {
                result.push('%');
                result.push_str(&format!("{b:02X}"));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::encode;

    #[test]
    fn spaces() {
        assert_eq!(encode("hello world"), "hello%20world");
    }

    #[test]
    fn special_chars() {
        assert_eq!(encode("foo=bar&baz"), "foo%3Dbar%26baz");
    }

    #[test]
    fn already_safe() {
        assert_eq!(encode("hello"), "hello");
    }

    #[test]
    fn empty() {
        assert_eq!(encode(""), "");
    }

    #[test]
    fn unreserved_chars() {
        assert_eq!(encode("a-b_c.d~e"), "a-b_c.d~e");
    }
}
