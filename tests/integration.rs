use std::io::Write;
use std::process::{Command, Output, Stdio};

fn bin(name: &str) -> Command {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    if cfg!(windows) {
        path.push("qk.exe");
    } else {
        path.push("qk");
    }
    let mut cmd = Command::new(path);
    cmd.arg(name);
    cmd
}

fn run(name: &str, args: &[&str]) -> Output {
    bin(name).args(args).output().unwrap()
}

fn stdout(o: &Output) -> String {
    String::from_utf8(o.stdout.clone())
        .unwrap()
        .trim_end()
        .to_string()
}

fn pipe_to(name: &str, args: &[&str], input: &str) -> Output {
    let mut child = bin(name)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

// ──────────────────────────────────────────────
// Boolean tools: exit codes
// ──────────────────────────────────────────────

#[test]
fn eq_true() {
    assert!(run("eq", &["abc", "abc"]).status.success());
}

#[test]
fn eq_false() {
    assert_eq!(run("eq", &["abc", "xyz"]).status.code(), Some(1));
}

#[test]
fn eq_stdin() {
    assert!(pipe_to("eq", &["hello"], "hello").status.success());
}

#[test]
fn contains_true() {
    assert!(run("contains", &["hello world", "world"]).status.success());
}

#[test]
fn contains_false() {
    assert_eq!(run("contains", &["hello", "xyz"]).status.code(), Some(1));
}

#[test]
fn contains_stdin() {
    assert!(
        pipe_to("contains", &["world"], "hello world")
            .status
            .success()
    );
}

#[test]
fn starts_true() {
    assert!(run("starts", &["hello", "hel"]).status.success());
}

#[test]
fn starts_false() {
    assert_eq!(run("starts", &["hello", "xyz"]).status.code(), Some(1));
}

#[test]
fn starts_stdin() {
    assert!(pipe_to("starts", &["hel"], "hello").status.success());
}

#[test]
fn ends_true() {
    assert!(run("ends", &["file.txt", ".txt"]).status.success());
}

#[test]
fn ends_false() {
    assert_eq!(run("ends", &["file.txt", ".rs"]).status.code(), Some(1));
}

#[test]
fn ends_stdin() {
    assert!(pipe_to("ends", &[".txt"], "file.txt").status.success());
}

#[test]
fn empty_true() {
    assert!(run("empty", &[""]).status.success());
}

#[test]
fn empty_false() {
    assert_eq!(run("empty", &["hi"]).status.code(), Some(1));
}

#[test]
fn isnum_int() {
    assert!(run("isnum", &["42"]).status.success());
}

#[test]
fn isnum_float() {
    assert!(run("isnum", &["3.14"]).status.success());
}

#[test]
fn isnum_not() {
    assert_eq!(run("isnum", &["abc"]).status.code(), Some(1));
}

#[test]
fn isnum_stdin() {
    assert!(pipe_to("isnum", &[], "42").status.success());
}

// ──────────────────────────────────────────────
// Output tools
// ──────────────────────────────────────────────

#[test]
fn len_basic() {
    assert_eq!(stdout(&run("len", &["hello"])), "5");
}

#[test]
fn len_unicode() {
    assert_eq!(stdout(&run("len", &["café"])), "4");
}

#[test]
fn len_stdin() {
    assert_eq!(stdout(&pipe_to("len", &[], "hello")), "5");
}

#[test]
fn trim_basic() {
    assert_eq!(stdout(&run("trim", &["  hi  "])), "hi");
}

#[test]
fn trim_stdin() {
    assert_eq!(stdout(&pipe_to("trim", &[], "  hi  ")), "hi");
}

#[test]
fn upper_basic() {
    assert_eq!(stdout(&run("upper", &["hello"])), "HELLO");
}

#[test]
fn upper_stdin() {
    assert_eq!(stdout(&pipe_to("upper", &[], "hello")), "HELLO");
}

#[test]
fn lower_basic() {
    assert_eq!(stdout(&run("lower", &["HELLO"])), "hello");
}

#[test]
fn lower_stdin() {
    assert_eq!(stdout(&pipe_to("lower", &[], "HELLO")), "hello");
}

#[test]
fn replace_basic() {
    assert_eq!(
        stdout(&run("replace", &["foo bar", "foo", "baz"])),
        "baz bar"
    );
}

#[test]
fn replace_stdin() {
    assert_eq!(
        stdout(&pipe_to("replace", &["foo", "baz"], "foo bar")),
        "baz bar"
    );
}

#[test]
fn rev_basic() {
    assert_eq!(stdout(&run("rev", &["hello"])), "olleh");
}

#[test]
fn rev_stdin() {
    assert_eq!(stdout(&pipe_to("rev", &[], "hello")), "olleh");
}

#[test]
fn slug_basic() {
    assert_eq!(stdout(&run("slug", &["Hello World!"])), "hello-world");
}

#[test]
fn slug_stdin() {
    assert_eq!(stdout(&pipe_to("slug", &[], "Hello World!")), "hello-world");
}

#[test]
fn between_basic() {
    assert_eq!(stdout(&run("between", &["a[b]c", "[", "]"])), "b");
}

#[test]
fn between_no_match() {
    assert_eq!(run("between", &["abc", "x", "y"]).status.code(), Some(2));
}

#[test]
fn sub_basic() {
    assert_eq!(stdout(&run("sub", &["hello world", "6", "5"])), "world");
}

#[test]
fn sub_stdin() {
    assert_eq!(stdout(&pipe_to("sub", &["6", "5"], "hello world")), "world");
}

#[test]
fn nth_basic() {
    assert_eq!(stdout(&run("nth", &["a,b,c", ",", "1"])), "b");
}

#[test]
fn nth_out_of_bounds() {
    assert_eq!(run("nth", &["a,b", ",", "5"]).status.code(), Some(2));
}

#[test]
fn match_found() {
    let o = run("match", &["hello123", r"\d+"]);
    assert!(o.status.success());
    assert_eq!(stdout(&o), "123");
}

#[test]
fn match_not_found() {
    assert_eq!(run("match", &["hello", r"\d+"]).status.code(), Some(1));
}

#[test]
fn matchall_basic() {
    assert_eq!(stdout(&run("matchall", &["a1b2c3", r"\d"])), "1\n2\n3");
}

#[test]
fn count_basic() {
    assert_eq!(stdout(&run("count", &["abab", "ab"])), "2");
}

#[test]
fn split_basic() {
    assert_eq!(stdout(&run("split", &["a,b,c", ","])), "a\nb\nc");
}

#[test]
fn chars_basic() {
    assert_eq!(stdout(&run("chars", &["hello"])), "h\ne\nl\nl\no");
}

#[test]
fn lines_stdin() {
    assert_eq!(stdout(&pipe_to("lines", &[], "a\nb\nc")), "3");
}

#[test]
fn uniq_basic() {
    assert_eq!(stdout(&run("uniq", &["abcabc"])), "abc");
}

#[test]
fn pad_basic() {
    assert_eq!(stdout(&run("pad", &["42", "5", "0"])), "00042");
}

#[test]
fn rpad_basic() {
    assert_eq!(stdout(&run("rpad", &["hi", "5", "."])), "hi...");
}

#[test]
fn repeat_basic() {
    assert_eq!(stdout(&run("repeat", &["ab", "3"])), "ababab");
}

#[test]
fn hash_basic() {
    assert_eq!(
        stdout(&run("hash", &["data"])),
        "3a6eb0790f39ac87c94f3856b2dd2c5d110e6811602261a9a923d3bb23adc8b7"
    );
}

#[test]
fn b64_encode() {
    assert_eq!(stdout(&run("b64", &["hello"])), "aGVsbG8=");
}

#[test]
fn b64_decode() {
    assert_eq!(stdout(&run("b64", &["-d", "aGVsbG8="])), "hello");
}

#[test]
fn b64_stdin() {
    assert_eq!(stdout(&pipe_to("b64", &[], "hello")), "aGVsbG8=");
}

#[test]
fn hex_encode() {
    assert_eq!(stdout(&run("hex", &["AB"])), "4142");
}

#[test]
fn hex_decode() {
    assert_eq!(stdout(&run("hex", &["-d", "4142"])), "AB");
}

#[test]
fn join_basic() {
    assert_eq!(stdout(&pipe_to("join", &[","], "a\nb\nc\n")), "a,b,c");
}

#[test]
fn join_space() {
    assert_eq!(stdout(&pipe_to("join", &[" "], "x\ny\nz")), "x y z");
}

#[test]
fn urlencode_basic() {
    assert_eq!(stdout(&run("urlencode", &["hello world"])), "hello%20world");
}

#[test]
fn urlencode_special() {
    assert_eq!(
        stdout(&run("urlencode", &["foo=bar&baz"])),
        "foo%3Dbar%26baz"
    );
}

#[test]
fn urlencode_stdin() {
    assert_eq!(
        stdout(&pipe_to("urlencode", &[], "hello world")),
        "hello%20world"
    );
}

#[test]
fn urldecode_basic() {
    assert_eq!(stdout(&run("urldecode", &["hello%20world"])), "hello world");
}

#[test]
fn urldecode_plus() {
    assert_eq!(stdout(&run("urldecode", &["hello+world"])), "hello world");
}

#[test]
fn urldecode_stdin() {
    assert_eq!(stdout(&pipe_to("urldecode", &[], "foo%3Dbar")), "foo=bar");
}

#[test]
fn urldecode_invalid() {
    assert_eq!(run("urldecode", &["%ZZ"]).status.code(), Some(2));
}

#[test]
fn ascii_basic() {
    assert_eq!(stdout(&run("ascii", &["café"])), "caf");
}

#[test]
fn ascii_all_ascii() {
    assert_eq!(stdout(&run("ascii", &["hello"])), "hello");
}

#[test]
fn ascii_stdin() {
    assert_eq!(stdout(&pipe_to("ascii", &[], "héllo")), "hllo");
}

#[test]
fn pipe_urlencode_roundtrip() {
    let encoded = stdout(&run("urlencode", &["hello world & more"]));
    let decoded = pipe_to("urldecode", &[], &encoded);
    assert_eq!(stdout(&decoded), "hello world & more");
}

// ──────────────────────────────────────────────
// Pipe chain tests
// ──────────────────────────────────────────────

#[test]
fn pipe_upper_then_contains() {
    let upper_out = run("upper", &["hello"]);
    let text = stdout(&upper_out);
    let result = pipe_to("contains", &["HELLO"], &text);
    assert!(result.status.success());
}

#[test]
fn pipe_split_then_join() {
    let split_out = stdout(&run("split", &["a,b,c", ","]));
    let join_out = pipe_to("join", &["-"], &format!("{split_out}\n"));
    assert_eq!(stdout(&join_out), "a-b-c");
}

#[test]
fn pipe_trim_then_upper() {
    let trimmed = stdout(&run("trim", &["  hello  "]));
    let result = pipe_to("upper", &[], &trimmed);
    assert_eq!(stdout(&result), "HELLO");
}

#[test]
fn pipe_replace_then_slug() {
    let replaced = stdout(&run("replace", &["Hello World", "World", "Rust"]));
    let result = pipe_to("slug", &[], &replaced);
    assert_eq!(stdout(&result), "hello-rust");
}

#[test]
fn pipe_b64_roundtrip() {
    let encoded = stdout(&run("b64", &["secret"]));
    let decoded = pipe_to("b64", &["-d"], &encoded);
    assert_eq!(stdout(&decoded), "secret");
}

#[test]
fn pipe_hex_roundtrip() {
    let encoded = stdout(&run("hex", &["hello"]));
    let decoded = pipe_to("hex", &["-d"], &encoded);
    assert_eq!(stdout(&decoded), "hello");
}

#[test]
fn pipe_match_then_len() {
    let matched = stdout(&run("match", &["abc123def", r"\d+"]));
    let result = pipe_to("len", &[], &matched);
    assert_eq!(stdout(&result), "3");
}

// ──────────────────────────────────────────────
// Help flag tests
// ──────────────────────────────────────────────

#[test]
fn help_flag() {
    let o = run("eq", &["--help"]);
    assert!(o.status.success());
    let stderr = String::from_utf8(o.stderr).unwrap();
    assert!(stderr.contains("eq"));
}

// ──────────────────────────────────────────────
// Error handling (exit code 2)
// ──────────────────────────────────────────────

#[test]
fn match_invalid_regex() {
    assert_eq!(run("match", &["abc", "["]).status.code(), Some(2));
}

#[test]
fn sub_invalid_offset() {
    assert_eq!(run("sub", &["hi", "abc", "1"]).status.code(), Some(2));
}

#[test]
fn unknown_command() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    if cfg!(windows) {
        path.push("qk.exe");
    } else {
        path.push("qk");
    }
    let o = Command::new(path).arg("nonexistent").output().unwrap();
    assert_eq!(o.status.code(), Some(2));
}
