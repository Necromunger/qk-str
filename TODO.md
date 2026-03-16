# TODO

## Done
- [x] Workspace structure with qk-common shared lib
- [x] All 31 tool binaries implemented
- [x] Unit tests for every crate (121 tests)
- [x] Integration tests with stdin piping, exit codes, pipe chains (68 tests)
- [x] Stdin convention: `resolve_context` handles argc-short-by-one fallback

## Remaining

### Ship it
- [x] Add .gitignore
- [x] Write README.md with tool table, install instructions, pipe examples
- [x] Set up GitHub Actions CI (dev/main split, main auto-publishes)
- [ ] Push to GitHub, verify CI passes, merge dev → main to trigger first publish

### Consider agent use
- [x] Created AGENTS.md — compact tool reference for LLM context windows

