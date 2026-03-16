use crate::common::{exit_err, out, resolve_context};

pub const USAGE: &str = "qk count <string> <substring> — count occurrences";

pub fn run(args: Vec<String>) {
    let (s, off) = resolve_context(&args, 2).unwrap_or_else(|| exit_err("missing argument"));
    let needle = args
        .get(off)
        .unwrap_or_else(|| exit_err("missing <substring>"));
    out(&s.matches(needle.as_str()).count().to_string());
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        assert_eq!("abab".matches("ab").count(), 2);
    }

    #[test]
    fn none() {
        assert_eq!("hello".matches("xyz").count(), 0);
    }

    #[test]
    fn single_char() {
        assert_eq!("aaa".matches("a").count(), 3);
    }

    #[test]
    fn empty_needle() {
        assert_eq!("hi".matches("").count(), 3);
    }
}
