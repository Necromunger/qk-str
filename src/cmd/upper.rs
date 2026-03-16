use crate::common::{exit_err, out, resolve_context};

pub const USAGE: &str = "qk upper <string> — convert to uppercase";

pub fn run(args: Vec<String>) {
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|| exit_err("missing argument"));
    out(&s.to_uppercase());
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        assert_eq!("hello".to_uppercase(), "HELLO");
    }

    #[test]
    fn mixed() {
        assert_eq!("Hello World".to_uppercase(), "HELLO WORLD");
    }

    #[test]
    fn unicode() {
        assert_eq!("café".to_uppercase(), "CAFÉ");
    }

    #[test]
    fn empty() {
        assert_eq!("".to_uppercase(), "");
    }
}
