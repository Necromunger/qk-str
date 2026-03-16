use qk_common::{exit_err, init, out, resolve_context};

fn main() {
    let args = init("trim <string> — trim whitespace from both ends");
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|_| exit_err("missing argument"));
    out(s.trim());
}

#[cfg(test)]
mod tests {
    #[test]
    fn both_sides() { assert_eq!("  hi  ".trim(), "hi"); }

    #[test]
    fn already_trimmed() { assert_eq!("hi".trim(), "hi"); }

    #[test]
    fn empty() { assert_eq!("".trim(), ""); }

    #[test]
    fn tabs_and_newlines() { assert_eq!("\t hello \n".trim(), "hello"); }
}
