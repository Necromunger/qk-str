use qk_common::{exit_err, init, out, resolve_context};

fn main() {
    let args = init("slug <string> — slugify the string");
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|_| exit_err("missing argument"));
    out(&slugify(&s));
}

fn slugify(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_dash = true; // avoid leading dash
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash {
            result.push('-');
            prev_dash = true;
        }
    }
    // remove trailing dash
    if result.ends_with('-') {
        result.pop();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::slugify;

    #[test]
    fn basic() { assert_eq!(slugify("Hello World!"), "hello-world"); }

    #[test]
    fn multiple_spaces() { assert_eq!(slugify("a  b  c"), "a-b-c"); }

    #[test]
    fn special_chars() { assert_eq!(slugify("foo@bar#baz"), "foo-bar-baz"); }

    #[test]
    fn empty() { assert_eq!(slugify(""), ""); }

    #[test]
    fn already_slug() { assert_eq!(slugify("hello-world"), "hello-world"); }
}
