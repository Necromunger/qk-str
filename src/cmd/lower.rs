use crate::common::{exit_err, out, resolve_context};

pub const USAGE: &str = "qk lower <string> — convert to lowercase";

pub fn run(args: Vec<String>) {
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|| exit_err("missing argument"));
    out(&s.to_lowercase());
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        assert_eq!("HELLO".to_lowercase(), "hello");
    }

    #[test]
    fn mixed() {
        assert_eq!("Hello World".to_lowercase(), "hello world");
    }

    #[test]
    fn unicode() {
        assert_eq!("CAFÉ".to_lowercase(), "café");
    }

    #[test]
    fn empty() {
        assert_eq!("".to_lowercase(), "");
    }
}
