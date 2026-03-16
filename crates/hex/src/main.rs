use qk_common::{exit_err, init, out, resolve_context};

fn main() {
    let args = init("hex [-d] <string> — hex encode (or decode with -d)");
    let decode = args.first().map(|s| s == "-d").unwrap_or(false);

    if decode {
        let (s, _) = resolve_context(&args[1..], 1).unwrap_or_else(|_| exit_err("missing argument"));
        match hex_decode(&s) {
            Some(text) => out(&text),
            None => exit_err("invalid hex string"),
        }
    } else {
        let (s, _) = resolve_context(&args, 1).unwrap_or_else(|_| exit_err("missing argument"));
        out(&hex_encode(&s));
    }
}

fn hex_encode(s: &str) -> String {
    s.bytes().map(|b| format!("{b:02x}")).collect()
}

fn hex_decode(s: &str) -> Option<String> {
    if s.len() % 2 != 0 {
        return None;
    }
    let bytes: Result<Vec<u8>, _> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect();
    bytes.ok().and_then(|b| String::from_utf8(b).ok())
}

#[cfg(test)]
mod tests {
    use super::{hex_encode, hex_decode};

    #[test]
    fn encode() { assert_eq!(hex_encode("AB"), "4142"); }

    #[test]
    fn decode() { assert_eq!(hex_decode("4142"), Some("AB".to_string())); }

    #[test]
    fn roundtrip() {
        let encoded = hex_encode("hello");
        assert_eq!(hex_decode(&encoded), Some("hello".to_string()));
    }

    #[test]
    fn empty() { assert_eq!(hex_encode(""), ""); }

    #[test]
    fn odd_length_decode() { assert_eq!(hex_decode("414"), None); }
}
