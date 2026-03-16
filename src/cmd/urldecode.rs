use crate::common::{exit_err, out, resolve_context};

pub const USAGE: &str = "qk urldecode <string> — decode percent-encoded string";

pub fn run(args: Vec<String>) {
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|| exit_err("missing argument"));
    match decode(&s) {
        Some(decoded) => out(&decoded),
        None => exit_err("invalid percent-encoding"),
    }
}

fn decode(s: &str) -> Option<String> {
    let mut bytes = Vec::with_capacity(s.len());
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let hi = chars.next()?;
            let lo = chars.next()?;
            let hex = [hi, lo];
            let hex_str = std::str::from_utf8(&hex).ok()?;
            let byte = u8::from_str_radix(hex_str, 16).ok()?;
            bytes.push(byte);
        } else if b == b'+' {
            bytes.push(b' ');
        } else {
            bytes.push(b);
        }
    }
    String::from_utf8(bytes).ok()
}

#[cfg(test)]
mod tests {
    use super::decode;

    #[test]
    fn spaces() {
        assert_eq!(decode("hello%20world"), Some("hello world".to_string()));
    }

    #[test]
    fn plus_as_space() {
        assert_eq!(decode("hello+world"), Some("hello world".to_string()));
    }

    #[test]
    fn special_chars() {
        assert_eq!(decode("foo%3Dbar%26baz"), Some("foo=bar&baz".to_string()));
    }

    #[test]
    fn already_decoded() {
        assert_eq!(decode("hello"), Some("hello".to_string()));
    }

    #[test]
    fn empty() {
        assert_eq!(decode(""), Some("".to_string()));
    }

    #[test]
    fn invalid_hex() {
        assert_eq!(decode("%ZZ"), None);
    }

    #[test]
    fn truncated() {
        assert_eq!(decode("%2"), None);
    }
}
