use qk_common::{exit_bool, exit_err, init, resolve_context};

fn main() {
    let args = init("empty <string> — exit 0 if string is empty");
    let (s, _) = resolve_context(&args, 1).unwrap_or_else(|_| exit_err("missing argument"));
    exit_bool(s.is_empty());
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty() { assert!("".is_empty()); }

    #[test]
    fn not_empty() { assert!(!"hello".is_empty()); }

    #[test]
    fn whitespace_not_empty() { assert!(!" ".is_empty()); }

    #[test]
    fn newline_not_empty() { assert!(!"\n".is_empty()); }
}
