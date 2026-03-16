use crate::common::{exit_err, out, resolve_context};

pub const USAGE: &str = "qk rpad <string> <width> [fill] — right-pad to width";

pub fn run(args: Vec<String>) {
    let (s, off) = resolve_context(&args, 2).unwrap_or_else(|| exit_err("missing argument"));
    let width: usize = args
        .get(off)
        .unwrap_or_else(|| exit_err("missing <width>"))
        .parse()
        .unwrap_or_else(|_| exit_err("invalid width"));
    let fill = args.get(off + 1).map(|s| s.as_str()).unwrap_or(" ");

    out(&right_pad(&s, width, fill));
}

fn right_pad(s: &str, width: usize, fill: &str) -> String {
    let char_count = s.chars().count();
    if char_count >= width {
        return s.to_string();
    }
    let pad_chars = width - char_count;
    let fill_chars: Vec<char> = fill.chars().collect();
    let mut result = s.to_string();
    for i in 0..pad_chars {
        result.push(fill_chars[i % fill_chars.len()]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::right_pad;

    #[test]
    fn basic() {
        assert_eq!(right_pad("hi", 5, "."), "hi...");
    }

    #[test]
    fn default_space() {
        assert_eq!(right_pad("hi", 5, " "), "hi   ");
    }

    #[test]
    fn no_pad_needed() {
        assert_eq!(right_pad("hello", 3, " "), "hello");
    }

    #[test]
    fn exact_width() {
        assert_eq!(right_pad("abc", 3, "."), "abc");
    }
}
