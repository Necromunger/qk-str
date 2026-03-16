use crate::common::{exit_bool, exit_err, resolve_context};

pub const USAGE: &str = "qk eq <a> <b> — exit 0 if strings are equal";

pub fn run(args: Vec<String>) {
    let (a, off) = resolve_context(&args, 2).unwrap_or_else(|| exit_err("missing argument"));
    let b = args
        .get(off)
        .unwrap_or_else(|| exit_err("missing second argument"));
    exit_bool(a == *b);
}

#[cfg(test)]
mod tests {
    #[test]
    fn equal() {
        assert!("hello" == "hello");
    }

    #[test]
    fn not_equal() {
        assert!("hello" != "world");
    }

    #[test]
    fn empty_strings() {
        assert!("" == "");
    }

    #[test]
    fn unicode() {
        assert!("café" == "café");
    }

    #[test]
    fn case_sensitive() {
        assert!("Hello" != "hello");
    }
}
