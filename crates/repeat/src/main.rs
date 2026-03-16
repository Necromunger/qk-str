use qk_common::{exit_err, init, out, resolve_context};

fn main() {
    let args = init("repeat <string> <count> — repeat string N times");
    let (s, off) = resolve_context(&args, 2).unwrap_or_else(|| exit_err("missing argument"));
    let n: usize = args
        .get(off)
        .unwrap_or_else(|| exit_err("missing <count>"))
        .parse()
        .unwrap_or_else(|_| exit_err("invalid count"));
    out(&s.repeat(n));
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        assert_eq!("ab".repeat(3), "ababab");
    }

    #[test]
    fn zero() {
        assert_eq!("ab".repeat(0), "");
    }

    #[test]
    fn one() {
        assert_eq!("hello".repeat(1), "hello");
    }

    #[test]
    fn empty() {
        assert_eq!("".repeat(5), "");
    }
}
