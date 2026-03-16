use qk_common::{exit_bool, exit_err, init, resolve_context};

fn main() {
    let args = init("eq <a> <b> — exit 0 if strings are equal");
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
