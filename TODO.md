# TODO

## Done
- [x] Workspace structure with qk-common shared lib
- [x] All 31 tool binaries implemented
- [x] Unit tests for every crate (121 tests)
- [x] Integration tests with stdin piping, exit codes, pipe chains (68 tests)
- [x] Stdin convention: `resolve_context` handles argc-short-by-one fallback

## Remaining

### Ship it
- [ ] Add .gitignore
- [ ] Write README.md with tool table, install instructions, pipe examples
- [ ] Set up GitHub Actions CI (test/clippy/fmt on 3 platforms + release builds)
- [ ] Register `qk-str` on crates.io (cargo publish placeholder 0.0.1)

### Pipe to clipboard
Consider how to pipe commands output directly into clipboard

`trim "  hello  " | cb`?

### Document all operations as list with description of use examples
Could be in the README as tail operations list after intro summary

### Consider agent use
Consider how AI agents know how to use this as a skill to save tokens

### Performance
Atomic operations have to consider or somehow validate their performance and/or fallbacks.
Fetch for example needs baked in timeout or error resolution.