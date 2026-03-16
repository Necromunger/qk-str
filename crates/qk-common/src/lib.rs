use std::io::Read;

/// Parse args, handle --help/-h with a one-line usage string.
/// Returns args excluding the binary name.
pub fn init(usage: &str) -> Vec<String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("{usage}");
        std::process::exit(0);
    }
    args
}

/// Read stdin to a string. Returns None if stdin is a terminal (no pipe).
pub fn read_stdin() -> Option<String> {
    if atty::is(atty::Stream::Stdin) {
        return None;
    }
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).ok()?;
    Some(buf)
}

fn trim_trailing_newline(mut s: String) -> String {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    s
}

/// Resolve context and return (context_value, param_offset).
/// `expected` is the total number of positional args the tool expects.
/// If `args.len() >= expected`, context comes from args[0] and params start at index 1.
/// If `args.len() == expected - 1`, stdin replaces context and params start at index 0.
pub fn resolve_context(args: &[String], expected: usize) -> Option<(String, usize)> {
    if args.len() >= expected {
        Some((args[0].clone(), 1))
    } else {
        read_stdin().map(|s| (trim_trailing_newline(s), 0))
    }
}

/// Print to stdout with trailing newline.
pub fn out(s: &str) {
    println!("{s}");
}

/// Exit with boolean status: 0 for true, 1 for false.
pub fn exit_bool(val: bool) -> ! {
    std::process::exit(if val { 0 } else { 1 });
}

/// Exit with error code 2 and a short stderr message.
pub fn exit_err(msg: &str) -> ! {
    eprintln!("{msg}");
    std::process::exit(2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_context_from_args() {
        let args = vec!["hello".to_string(), "world".to_string()];
        let (val, off) = resolve_context(&args, 2).unwrap();
        assert_eq!(val, "hello");
        assert_eq!(off, 1);
    }

    #[test]
    fn test_resolve_context_single_arg_tool() {
        let args = vec!["hello".to_string()];
        let (val, off) = resolve_context(&args, 1).unwrap();
        assert_eq!(val, "hello");
        assert_eq!(off, 1);
    }
}
