use crate::common::{exit_bool, exit_err, resolve_context};

pub const USAGE: &str = "qk starts <string> <prefix> — exit 0 if string starts with prefix";

pub fn run(args: Vec<String>) {
    let (s, off) = resolve_context(&args, 2).unwrap_or_else(|| exit_err("missing argument"));
    let prefix = args.get(off).unwrap_or_else(|| exit_err("missing prefix"));
    exit_bool(s.starts_with(prefix.as_str()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn matches() {
        assert!("hello".starts_with("hel"));
    }

    #[test]
    fn no_match() {
        assert!(!"hello".starts_with("xyz"));
    }

    #[test]
    fn empty_prefix() {
        assert!("hello".starts_with(""));
    }

    #[test]
    fn full_string() {
        assert!("hello".starts_with("hello"));
    }
}
