use crate::common::{exit_bool, exit_err, resolve_context};

pub const USAGE: &str = "qk ends <string> <suffix> — exit 0 if string ends with suffix";

pub fn run(args: Vec<String>) {
    let (s, off) = resolve_context(&args, 2).unwrap_or_else(|| exit_err("missing argument"));
    let suffix = args.get(off).unwrap_or_else(|| exit_err("missing suffix"));
    exit_bool(s.ends_with(suffix.as_str()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn matches() {
        assert!("file.txt".ends_with(".txt"));
    }

    #[test]
    fn no_match() {
        assert!(!"file.txt".ends_with(".rs"));
    }

    #[test]
    fn empty_suffix() {
        assert!("hello".ends_with(""));
    }

    #[test]
    fn full_string() {
        assert!("hello".ends_with("hello"));
    }
}
