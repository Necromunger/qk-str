# QK-STR — Quick String Toolkit

## Overview

A suite of ~31 small, fast, composable CLI binaries for string operations on the terminal. Each tool does one thing. No flags, no ceremony. Designed to fill the missing "string primitive" layer in shell interfaces.

Published as a single Rust cargo workspace. Distributed via `cargo install qk-str` (source) and GitHub Releases (pre-built binaries per platform).

**Crate name `qk-str` is confirmed available on crates.io. Register it early with a minimal `cargo publish` (even a placeholder version 0.0.1) before building out, to secure the name.**

## Core Design Rules

1. **arg1 is always the context** (the thing being operated on), **arg2+ are parameters**
2. **stdin replaces arg1 when argc is short by one** — this is how piping works: `fetch "url" | contains "needle"` means stdin becomes the haystack
3. **Boolean tools** (eq, contains, starts, ends, isnum, empty, match-like) exit 0 for true, 1 for false. No stdout unless explicitly needed.
4. **Output tools** print to stdout with a trailing newline. No decoration, no labels.
5. **No flags, no `--help` walls, no `--input` nonsense.** If the user passes `--help` or `-h`, print a single line of usage. That's it.
6. **Fail silently with exit code 2** on bad input (wrong arg count, invalid regex, network error). Print a short stderr message only.
7. **UTF-8 throughout.** No codepage games.

## Project Structure

```
qk-str/
├── Cargo.toml          # workspace root
├── CLAUDE.md           # this file
├── README.md           # public docs
├── LICENSE             # MIT
├── crates/
│   ├── qk-common/      # shared lib: stdin reading, arg parsing, exit helpers
│   │   └── src/lib.rs
│   ├── eq/
│   │   └── src/main.rs
│   ├── contains/
│   │   └── src/main.rs
│   ├── starts/
│   │   └── src/main.rs
│   ├── ends/
│   │   └── src/main.rs
│   ├── len/
│   │   └── src/main.rs
│   ├── trim/
│   │   └── src/main.rs
│   ├── upper/
│   │   └── src/main.rs
│   ├── lower/
│   │   └── src/main.rs
│   ├── replace/
│   │   └── src/main.rs
│   ├── between/
│   │   └── src/main.rs
│   ├── split/
│   │   └── src/main.rs
│   ├── count/
│   │   └── src/main.rs
│   ├── rev/
│   │   └── src/main.rs
│   ├── repeat/
│   │   └── src/main.rs
│   ├── pad/
│   │   └── src/main.rs
│   ├── rpad/
│   │   └── src/main.rs
│   ├── sub/
│   │   └── src/main.rs
│   ├── lines/
│   │   └── src/main.rs
│   ├── nth/
│   │   └── src/main.rs
│   ├── join/
│   │   └── src/main.rs
│   ├── uniq/
│   │   └── src/main.rs
│   ├── chars/
│   │   └── src/main.rs
│   ├── isnum/
│   │   └── src/main.rs
│   ├── empty/
│   │   └── src/main.rs
│   ├── match/
│   │   └── src/main.rs
│   ├── matchall/
│   │   └── src/main.rs
│   ├── slug/
│   │   └── src/main.rs
│   ├── hash/
│   │   └── src/main.rs
│   ├── b64/
│   │   └── src/main.rs
│   ├── hex/
│   │   └── src/main.rs
│   └── fetch/
│       └── src/main.rs
└── tests/
    └── integration.rs   # end-to-end CLI tests via std::process::Command
```

## Shared Library: qk-common

Provides:

```rust
/// Read stdin to string (blocking, with timeout awareness)
pub fn read_stdin() -> Option<String>;

/// Resolve context: if `args` has enough elements, return args[index].
/// Otherwise fall back to stdin. Trims trailing newline from stdin.
pub fn resolve_context(args: &[String], index: usize) -> Result<String, ()>;

/// Print to stdout with trailing newline
pub fn out(s: &str);

/// Exit true (0) or false (1)
pub fn exit_bool(val: bool);

/// Exit with error (2) and stderr message
pub fn exit_err(msg: &str) -> !;

/// One-line usage on --help/-h, otherwise return args (excluding binary name)
pub fn init(usage: &str) -> Vec<String>;
```

`resolve_context` is the core of the stdin convention. Every binary calls it to get its primary operand.

## Tool Specifications

### Boolean tools (exit 0 = true, 1 = false)

| Tool | Usage | Behaviour |
|------|-------|-----------|
| eq | `eq "a" "b"` | Exact string equality |
| contains | `contains "haystack" "needle"` | Substring presence |
| starts | `starts "hello" "hel"` | Prefix check |
| ends | `ends "file.txt" ".txt"` | Suffix check |
| isnum | `isnum "42.5"` | Parseable as f64 |
| empty | `empty ""` | True if zero-length (after no trimming) |

### Output tools (print to stdout)

| Tool | Usage | Output |
|------|-------|--------|
| len | `len "hello"` | `5` |
| trim | `trim "  hi  "` | `hi` |
| upper | `upper "hello"` | `HELLO` |
| lower | `lower "Hello"` | `hello` |
| replace | `replace "foo bar" "foo" "baz"` | `baz bar` (all occurrences) |
| between | `between "a[b]c" "[" "]"` | `b` (first match) |
| split | `split "a,b,c" ","` | `a\nb\nc` (one per line) |
| count | `count "abab" "ab"` | `2` |
| rev | `rev "hello"` | `olleh` |
| repeat | `repeat "ab" 3` | `ababab` |
| pad | `pad "42" 5 "0"` | `00042` (left-pad, 3rd arg optional, default space) |
| rpad | `rpad "hi" 5 "."` | `hi...` (right-pad, 3rd arg optional, default space) |
| sub | `sub "hello world" 6 5` | `world` (offset, length) |
| lines | `lines "file.txt"` or stdin | Line count as number |
| nth | `nth "a,b,c" "," 1` | `b` (0-indexed) |
| join | stdin lines \| `join ","` | `a,b,c` |
| uniq | `uniq "abcabc"` | `abc` (char-level ordered dedupe) |
| chars | `chars "hello"` | `h\ne\nl\nl\no` (one per line) |
| match | `match "hello123" "\d+"` | `123` (first regex match, exit 1 if no match) |
| matchall | `matchall "a1b2" "\d"` | `1\n2` (all matches, one per line) |
| slug | `slug "Hello World!"` | `hello-world` |
| hash | `hash "data"` | SHA-256 hex string |
| b64 | `b64 "hello"` | Base64 encoded. `b64 -d "aGVsbG8="` decodes. **Only exception to no-flags rule.** |
| hex | `hex "AB"` | `4142`. `hex -d "4142"` → `AB`. **Only exception to no-flags rule.** |
| fetch | `fetch "https://example.com"` | HTTP GET, body to stdout. Follows redirects. |

### Special: join

`join` is stdin-only by design. It takes one arg (the delimiter) and reads lines from stdin.

### Special: lines

`lines` accepts either a filename arg OR stdin. If arg looks like a readable file path, counts lines in that file. Otherwise counts lines in stdin.

### Special: fetch

Uses `ureq` or `reqwest` (blocking). Follows redirects (max 10). Timeout 30 seconds. Stderr on failure, exit 2.

## Dependencies

Minimal. Target:

- `regex` — for match/matchall only
- `sha2` — for hash
- `base64` — for b64
- `ureq` — for fetch (blocking, minimal footprint, no tokio)

Everything else is stdlib. Do not introduce dependencies for things Rust can do natively.

## Testing

### Unit tests

Each crate has `#[cfg(test)]` module in main.rs with at least 3-5 test cases covering:
- Happy path
- Edge case (empty string, unicode, very long input)
- Boundary behaviour (e.g. `between` with no match, `nth` out of bounds)

### Integration tests

`tests/integration.rs` uses `std::process::Command` to invoke the compiled binaries and assert on:
- stdout content
- exit codes (especially for boolean tools)
- **Pipe chains**: e.g. echo "hello world" | contains "world" → exit 0
- **Stdin fallback**: verify that piped input works as context replacement

Minimum 3 integration tests per binary. At least 5 pipe-chain tests combining multiple tools.

### Test commands

```bash
cargo test --workspace          # all unit + integration
cargo test -p eq                # single crate
cargo test --test integration   # integration only
```

## Build & Release

### Local dev

```bash
cargo build --workspace
cargo test --workspace
```

All binaries land in `target/debug/` (or `target/release/` with `--release`).

### CI (GitHub Actions)

On push to `main`:
- `cargo test --workspace` on ubuntu-latest, windows-latest, macos-latest
- `cargo clippy --workspace -- -D warnings`
- `cargo fmt --check`

On tag `v*`:
- Build release binaries for:
  - x86_64-unknown-linux-gnu
  - x86_64-pc-windows-msvc
  - aarch64-apple-darwin
  - x86_64-apple-darwin
- Package each as `qk-str-{version}-{target}.zip` containing all binaries
- Create GitHub Release with attached zips
- `cargo publish` to crates.io

Use `cargo-dist` or a manual matrix build. Prefer `cargo-dist` if it supports the workspace layout cleanly.

### Versioning

Semver. All crates share a single version number managed at workspace level. Bumping version means bumping all crates together.

## README.md

Should include:
- One-line pitch: "31 string commands for your terminal. No flags. No ceremony."
- Install section: `cargo install qk-str` and GitHub Releases link
- Full table of all tools with one-line examples
- Pipe composition examples showing 3-4 real chains
- The stdin convention explained in 2 sentences

## Implementation Order

0. **Register `qk-str` on crates.io** — `cargo login`, publish a 0.0.1 placeholder to lock the name
1. `qk-common` shared lib
2. Boolean tools: eq, contains, starts, ends, empty, isnum
3. Transform tools: trim, upper, lower, replace, rev, slug
4. Extract tools: between, sub, nth, match, matchall
5. Counting/splitting: len, count, split, chars, lines, uniq
6. Formatting: pad, rpad, repeat
7. Encoding: hash, b64, hex
8. IO: join, fetch
9. Integration tests for pipe chains
10. CI pipeline
11. README + publish

Each phase should be fully tested before moving to the next.

# Cleanup

Finalize and trim this file when complete