use qk_common::{exit_bool, exit_err, init, resolve_context};

fn main() {
    let args = init("contains <haystack> <needle> — exit 0 if haystack contains needle");
    let (haystack, off) = resolve_context(&args, 2).unwrap_or_else(|_| exit_err("missing argument"));
    let needle = args.get(off).unwrap_or_else(|| exit_err("missing needle"));
    exit_bool(haystack.contains(needle.as_str()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn found() { assert!("hello world".contains("world")); }

    #[test]
    fn not_found() { assert!(!"hello".contains("xyz")); }

    #[test]
    fn empty_needle() { assert!("hello".contains("")); }

    #[test]
    fn unicode() { assert!("café latte".contains("é")); }
}
