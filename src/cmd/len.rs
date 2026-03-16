use crate::common::{exit_err, out, resolve_context};

pub const USAGE: &str = "qk len <string> — print character count";

pub fn run(args: Vec<String>) {
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|| exit_err("missing argument"));
    out(&s.chars().count().to_string());
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        assert_eq!("hello".chars().count(), 5);
    }

    #[test]
    fn empty() {
        assert_eq!("".chars().count(), 0);
    }

    #[test]
    fn unicode() {
        assert_eq!("café".chars().count(), 4);
    }

    #[test]
    fn spaces() {
        assert_eq!("a b c".chars().count(), 5);
    }
}
