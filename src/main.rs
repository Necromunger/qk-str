mod cmd;
mod common;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("qk — quick string toolkit");
        eprintln!("Usage: qk <command> [args...]");
        eprintln!("Commands: eq, contains, starts, ends, empty, isnum, len, trim, upper, lower,");
        eprintln!("  replace, rev, slug, between, sub, nth, match, matchall, count, split, chars,");
        eprintln!("  lines, uniq, pad, rpad, repeat, hash, b64, hex, join, fetch, urlencode,");
        eprintln!("  urldecode, ascii");
        std::process::exit(2);
    }

    let subcommand = &args[1];
    let sub_args: Vec<String> = args[2..].to_vec();

    // Global --help
    if subcommand == "--help" || subcommand == "-h" {
        eprintln!("qk — quick string toolkit");
        eprintln!("Usage: qk <command> [args...]");
        eprintln!();
        eprintln!("Commands:");
        eprintln!("  eq, contains, starts, ends, empty, isnum  Boolean (exit 0/1)");
        eprintln!("  len, trim, upper, lower, replace, rev      Transform");
        eprintln!("  slug, between, sub, nth, match, matchall   Extract");
        eprintln!("  count, split, chars, lines, uniq            Split/count");
        eprintln!("  pad, rpad, repeat                           Format");
        eprintln!("  hash, b64, hex, urlencode, urldecode         Encode");
        eprintln!("  join, fetch                                 IO");
        eprintln!("  ascii                                        Sanitize");
        std::process::exit(0);
    }

    // Per-command --help
    if sub_args.iter().any(|a| a == "--help" || a == "-h") {
        let usage = match subcommand.as_str() {
            "eq" => cmd::eq::USAGE,
            "contains" => cmd::contains::USAGE,
            "starts" => cmd::starts::USAGE,
            "ends" => cmd::ends::USAGE,
            "empty" => cmd::empty::USAGE,
            "isnum" => cmd::isnum::USAGE,
            "len" => cmd::len::USAGE,
            "trim" => cmd::trim::USAGE,
            "upper" => cmd::upper::USAGE,
            "lower" => cmd::lower::USAGE,
            "replace" => cmd::replace::USAGE,
            "rev" => cmd::rev::USAGE,
            "slug" => cmd::slug::USAGE,
            "between" => cmd::between::USAGE,
            "sub" => cmd::sub::USAGE,
            "nth" => cmd::nth::USAGE,
            "match" => cmd::match_cmd::USAGE,
            "matchall" => cmd::matchall::USAGE,
            "count" => cmd::count::USAGE,
            "split" => cmd::split::USAGE,
            "chars" => cmd::chars::USAGE,
            "lines" => cmd::lines::USAGE,
            "uniq" => cmd::uniq::USAGE,
            "pad" => cmd::pad::USAGE,
            "rpad" => cmd::rpad::USAGE,
            "repeat" => cmd::repeat::USAGE,
            "hash" => cmd::hash::USAGE,
            "b64" => cmd::b64::USAGE,
            "hex" => cmd::hex::USAGE,
            "join" => cmd::join::USAGE,
            "fetch" => cmd::fetch::USAGE,
            "urlencode" => cmd::urlencode::USAGE,
            "urldecode" => cmd::urldecode::USAGE,
            "ascii" => cmd::ascii::USAGE,
            _ => {
                eprintln!("unknown command: {subcommand}");
                std::process::exit(2);
            }
        };
        eprintln!("{usage}");
        std::process::exit(0);
    }

    match subcommand.as_str() {
        "eq" => cmd::eq::run(sub_args),
        "contains" => cmd::contains::run(sub_args),
        "starts" => cmd::starts::run(sub_args),
        "ends" => cmd::ends::run(sub_args),
        "empty" => cmd::empty::run(sub_args),
        "isnum" => cmd::isnum::run(sub_args),
        "len" => cmd::len::run(sub_args),
        "trim" => cmd::trim::run(sub_args),
        "upper" => cmd::upper::run(sub_args),
        "lower" => cmd::lower::run(sub_args),
        "replace" => cmd::replace::run(sub_args),
        "rev" => cmd::rev::run(sub_args),
        "slug" => cmd::slug::run(sub_args),
        "between" => cmd::between::run(sub_args),
        "sub" => cmd::sub::run(sub_args),
        "nth" => cmd::nth::run(sub_args),
        "match" => cmd::match_cmd::run(sub_args),
        "matchall" => cmd::matchall::run(sub_args),
        "count" => cmd::count::run(sub_args),
        "split" => cmd::split::run(sub_args),
        "chars" => cmd::chars::run(sub_args),
        "lines" => cmd::lines::run(sub_args),
        "uniq" => cmd::uniq::run(sub_args),
        "pad" => cmd::pad::run(sub_args),
        "rpad" => cmd::rpad::run(sub_args),
        "repeat" => cmd::repeat::run(sub_args),
        "hash" => cmd::hash::run(sub_args),
        "b64" => cmd::b64::run(sub_args),
        "hex" => cmd::hex::run(sub_args),
        "join" => cmd::join::run(sub_args),
        "fetch" => cmd::fetch::run(sub_args),
        "urlencode" => cmd::urlencode::run(sub_args),
        "urldecode" => cmd::urldecode::run(sub_args),
        "ascii" => cmd::ascii::run(sub_args),
        _ => {
            eprintln!("unknown command: {subcommand}");
            std::process::exit(2);
        }
    }
}
