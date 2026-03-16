use qk_common::{exit_bool, exit_err, init, resolve_context};

fn main() {
    let args = init("isnum <string> — exit 0 if string is a valid number");
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|| exit_err("missing argument"));
    exit_bool(s.parse::<f64>().is_ok());
}

#[cfg(test)]
mod tests {
    #[test]
    fn integer() {
        assert!("42".parse::<f64>().is_ok());
    }

    #[test]
    fn float() {
        assert!("42.5".parse::<f64>().is_ok());
    }

    #[test]
    fn negative() {
        assert!("-3.14".parse::<f64>().is_ok());
    }

    #[test]
    fn not_number() {
        assert!("hello".parse::<f64>().is_err());
    }

    #[test]
    fn empty_string() {
        assert!("".parse::<f64>().is_err());
    }
}
