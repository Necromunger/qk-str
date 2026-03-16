use qk_common::{exit_err, init, out, resolve_context};

fn left_pad(s: &str, width: usize, fill: &str) -> String {
    let char_count = s.chars().count();
    if char_count >= width {
        return s.to_string();
    }
    let pad_chars = width - char_count;
    let fill_chars: Vec<char> = fill.chars().collect();
    let mut result = String::new();
    for i in 0..pad_chars {
        result.push(fill_chars[i % fill_chars.len()]);
    }
    result.push_str(s);
    result
}

fn main() {
    let args = init("pad <string> <width> [fill] — left-pad to width");
    let (s, off) = resolve_context(&args, 2).unwrap_or_else(|_| exit_err("missing argument"));
    let width: usize = args.get(off)
        .unwrap_or_else(|| exit_err("missing <width>"))
        .parse()
        .unwrap_or_else(|_| exit_err("invalid width"));
    let fill = args.get(off + 1).map(|s| s.as_str()).unwrap_or(" ");

    out(&left_pad(&s, width, fill));
}

#[cfg(test)]
mod tests {
    use super::left_pad;

    #[test]
    fn basic() { assert_eq!(left_pad("42", 5, "0"), "00042"); }

    #[test]
    fn default_space() { assert_eq!(left_pad("hi", 5, " "), "   hi"); }

    #[test]
    fn no_pad_needed() { assert_eq!(left_pad("hello", 3, " "), "hello"); }

    #[test]
    fn exact_width() { assert_eq!(left_pad("abc", 3, " "), "abc"); }
}
