use qk_common::{exit_err, init, out, resolve_context};

fn main() {
    let args = init("nth <string> <delimiter> <index> — get nth element (0-indexed)");
    let (s, off) = resolve_context(&args, 3).unwrap_or_else(|| exit_err("missing argument"));
    let delim = args
        .get(off)
        .unwrap_or_else(|| exit_err("missing <delimiter>"));
    let index: usize = args
        .get(off + 1)
        .unwrap_or_else(|| exit_err("missing <index>"))
        .parse()
        .unwrap_or_else(|_| exit_err("invalid index"));

    let parts: Vec<&str> = s.split(delim.as_str()).collect();
    match parts.get(index) {
        Some(val) => out(val),
        None => exit_err("index out of bounds"),
    }
}

#[cfg(test)]
mod tests {
    fn nth(s: &str, delim: &str, index: usize) -> Option<String> {
        s.split(delim).nth(index).map(|v| v.to_string())
    }

    #[test]
    fn basic() {
        assert_eq!(nth("a,b,c", ",", 1).unwrap(), "b");
    }

    #[test]
    fn first() {
        assert_eq!(nth("a,b,c", ",", 0).unwrap(), "a");
    }

    #[test]
    fn last() {
        assert_eq!(nth("a,b,c", ",", 2).unwrap(), "c");
    }

    #[test]
    fn out_of_bounds() {
        assert!(nth("a,b", ",", 5).is_none());
    }
}
