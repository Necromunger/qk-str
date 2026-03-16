# QK-STR

Single binary `qk` with 31 string subcommands. Published on crates.io as `qk-str`.

## Design Rules

1. `qk <command> <context> [params...]` — arg1 is always the context
2. Stdin replaces arg1 when argc is short by one — this is how piping works
3. Boolean tools print `true`/`false` and exit 0/1
4. Output tools print to stdout with trailing newline
5. Errors exit 2 with short stderr message
6. No flags except `b64 -d` and `hex -d`
7. UTF-8 throughout

## Structure

```
src/
  main.rs        # dispatcher: matches argv[1] to cmd::*
  common.rs      # resolve_context, read_stdin, out, exit_bool, exit_err
  cmd/
    mod.rs
    eq.rs, contains.rs, ... (31 modules, match_cmd.rs for "match")
tests/
  integration.rs # 69 tests via std::process::Command
```

Each command module exports `pub const USAGE` and `pub fn run(args: Vec<String>)`.

## Commands

```
cargo build         # binary at target/debug/qk
cargo test          # 125 unit + 69 integration tests
cargo clippy -- -D warnings
```

## CI/CD

- `.github/workflows/ci.yml` — tests on push to dev/main (3 platforms)
- `.github/workflows/release.yml` — on push to main (src/Cargo changes only): test, cargo publish, build release binaries for 4 targets, create GitHub Release

## Dependencies

Only: `atty`, `regex`, `sha2`, `base64`, `ureq` (native-tls). Keep minimal.
