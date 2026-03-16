use qk_common::{exit_err, init, out, resolve_context};

fn main() {
    let args = init("rev <string> — reverse the string");
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|_| exit_err("missing argument"));
    out(&s.chars().rev().collect::<String>());
}

#[cfg(test)]
mod tests {
    fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }

    #[test]
    fn basic() { assert_eq!(reverse("hello"), "olleh"); }

    #[test]
    fn palindrome() { assert_eq!(reverse("racecar"), "racecar"); }

    #[test]
    fn empty() { assert_eq!(reverse(""), ""); }

    #[test]
    fn unicode() { assert_eq!(reverse("café"), "éfac"); }
}
