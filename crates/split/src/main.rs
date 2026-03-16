use qk_common::{exit_err, init, out, resolve_context};

fn main() {
    let args = init("split <string> <delimiter> — split into lines");
    let (s, off) = resolve_context(&args, 2).unwrap_or_else(|_| exit_err("missing argument"));
    let delim = args.get(off).unwrap_or_else(|| exit_err("missing <delimiter>"));
    for part in s.split(delim.as_str()) {
        out(part);
    }
}

#[cfg(test)]
mod tests {
    fn split_to_lines(s: &str, delim: &str) -> String {
        s.split(delim).collect::<Vec<_>>().join("\n")
    }

    #[test]
    fn basic() { assert_eq!(split_to_lines("a,b,c", ","), "a\nb\nc"); }

    #[test]
    fn no_delimiter() { assert_eq!(split_to_lines("hello", ","), "hello"); }

    #[test]
    fn multi_char_delim() { assert_eq!(split_to_lines("a::b::c", "::"), "a\nb\nc"); }
}
