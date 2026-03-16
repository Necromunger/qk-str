use crate::common::{exit_err, out, resolve_context};

pub const USAGE: &str = "qk replace <string> <from> <to> — replace all occurrences";

pub fn run(args: Vec<String>) {
    let (s, off) = resolve_context(&args, 3).unwrap_or_else(|| exit_err("missing argument"));
    let from = args.get(off).unwrap_or_else(|| exit_err("missing <from>"));
    let to = args
        .get(off + 1)
        .unwrap_or_else(|| exit_err("missing <to>"));
    out(&s.replace(from.as_str(), to.as_str()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        assert_eq!("foo bar".replace("foo", "baz"), "baz bar");
    }

    #[test]
    fn multiple() {
        assert_eq!("abab".replace("ab", "x"), "xx");
    }

    #[test]
    fn no_match() {
        assert_eq!("hello".replace("xyz", "abc"), "hello");
    }

    #[test]
    fn empty_replacement() {
        assert_eq!("hello".replace("l", ""), "heo");
    }
}
