use qk_common::{exit_err, init, out, resolve_context};
use sha2::{Digest, Sha256};

fn main() {
    let args = init("hash <string> — SHA-256 hex digest");
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|_| exit_err("missing argument"));
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();
    out(&format!("{result:x}"));
}

#[cfg(test)]
mod tests {
    use sha2::{Digest, Sha256};

    fn sha256(s: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    #[test]
    fn known_hash() {
        assert_eq!(sha256("data"), "3a6eb0790f39ac87c94f3856b2dd2c5d110e6811602261a9a923d3bb23adc8b7");
    }

    #[test]
    fn empty() {
        assert_eq!(sha256(""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn deterministic() { assert_eq!(sha256("test"), sha256("test")); }
}
