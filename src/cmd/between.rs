use crate::common::{exit_err, out, resolve_context};

pub const USAGE: &str = "qk between <string> <start> <end> — extract text between delimiters";

pub fn run(args: Vec<String>) {
    let (s, off) = resolve_context(&args, 3).unwrap_or_else(|| exit_err("missing argument"));
    let start = args.get(off).unwrap_or_else(|| exit_err("missing <start>"));
    let end = args
        .get(off + 1)
        .unwrap_or_else(|| exit_err("missing <end>"));

    if let Some(result) = extract_between(&s, start, end) {
        out(&result);
    } else {
        exit_err("no match found");
    }
}

fn extract_between(s: &str, start: &str, end: &str) -> Option<String> {
    let start_pos = s.find(start)? + start.len();
    let end_pos = s[start_pos..].find(end)? + start_pos;
    Some(s[start_pos..end_pos].to_string())
}

#[cfg(test)]
mod tests {
    use super::extract_between;

    #[test]
    fn basic() {
        assert_eq!(extract_between("a[b]c", "[", "]"), Some("b".to_string()));
    }

    #[test]
    fn html_tags() {
        assert_eq!(
            extract_between("<b>hello</b>", "<b>", "</b>"),
            Some("hello".to_string())
        );
    }

    #[test]
    fn no_match() {
        assert_eq!(extract_between("abc", "x", "y"), None);
    }

    #[test]
    fn multiple_takes_first() {
        assert_eq!(extract_between("[a][b]", "[", "]"), Some("a".to_string()));
    }

    #[test]
    fn empty_between() {
        assert_eq!(extract_between("[]", "[", "]"), Some("".to_string()));
    }
}
