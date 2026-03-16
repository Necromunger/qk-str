use base64::{Engine, engine::general_purpose::STANDARD};
use qk_common::{exit_err, init, out, resolve_context};

fn main() {
    let args = init("b64 [-d] <string> — base64 encode (or decode with -d)");
    let decode = args.first().map(|s| s == "-d").unwrap_or(false);

    if decode {
        let (s, _) = resolve_context(&args[1..], 1).unwrap_or_else(|_| exit_err("missing argument"));
        match STANDARD.decode(s.as_bytes()) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(text) => out(&text),
                Err(_) => exit_err("decoded bytes are not valid UTF-8"),
            },
            Err(_) => exit_err("invalid base64"),
        }
    } else {
        let (s, _) = resolve_context(&args, 1).unwrap_or_else(|_| exit_err("missing argument"));
        out(&STANDARD.encode(s.as_bytes()));
    }
}

#[cfg(test)]
mod tests {
    use base64::{Engine, engine::general_purpose::STANDARD};

    #[test]
    fn encode() { assert_eq!(STANDARD.encode("hello"), "aGVsbG8="); }

    #[test]
    fn decode() {
        let bytes = STANDARD.decode("aGVsbG8=").unwrap();
        assert_eq!(String::from_utf8(bytes).unwrap(), "hello");
    }

    #[test]
    fn roundtrip() {
        let encoded = STANDARD.encode("test data");
        let decoded = STANDARD.decode(&encoded).unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), "test data");
    }

    #[test]
    fn empty() { assert_eq!(STANDARD.encode(""), ""); }
}
