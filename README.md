# qk-str

31 string commands for your terminal. No flags. No ceremony.

## Install

```bash
cargo install qk-str
```

Or grab a binary from [GitHub Releases](https://github.com/Necromunger/qk-str/releases).

## How it works

Every tool takes **arg1 as the context** (the thing being operated on). If you pipe stdin, it replaces arg1 — that's how composition works:

```bash
echo "  Hello World  " | trim | upper
# HELLO WORLD

split "a,b,c" "," | join "-"
# a-b-c

echo "Hello World!" | slug
# hello-world

echo "secret" | b64 | b64 -d
# secret
```

Boolean tools exit 0 (true) or 1 (false) — perfect for `if`/`&&`:

```bash
contains "hello world" "world" && echo "found it"
isnum "42.5" && echo "it's a number"
```

## Tools

| Tool | Example | Output |
|------|---------|--------|
| eq | `eq "a" "b"` | exit 0 if equal |
| contains | `contains "haystack" "needle"` | exit 0 if found |
| starts | `starts "hello" "hel"` | exit 0 if prefix matches |
| ends | `ends "file.txt" ".txt"` | exit 0 if suffix matches |
| isnum | `isnum "42.5"` | exit 0 if parseable as number |
| empty | `empty ""` | exit 0 if zero-length |
| len | `len "hello"` | `5` |
| trim | `trim "  hi  "` | `hi` |
| upper | `upper "hello"` | `HELLO` |
| lower | `lower "Hello"` | `hello` |
| replace | `replace "foo bar" "foo" "baz"` | `baz bar` |
| between | `between "a[b]c" "[" "]"` | `b` |
| split | `split "a,b,c" ","` | `a\nb\nc` |
| count | `count "abab" "ab"` | `2` |
| rev | `rev "hello"` | `olleh` |
| repeat | `repeat "ab" 3` | `ababab` |
| pad | `pad "42" 5 "0"` | `00042` |
| rpad | `rpad "hi" 5 "."` | `hi...` |
| sub | `sub "hello world" 6 5` | `world` |
| lines | `lines file.txt` | line count |
| nth | `nth "a,b,c" "," 1` | `b` |
| join | `... \| join ","` | joins stdin lines |
| uniq | `uniq "abcabc"` | `abc` |
| chars | `chars "hello"` | `h\ne\nl\nl\no` |
| match | `match "hello123" "\d+"` | `123` |
| matchall | `matchall "a1b2" "\d"` | `1\n2` |
| slug | `slug "Hello World!"` | `hello-world` |
| hash | `hash "data"` | SHA-256 hex |
| b64 | `b64 "hello"` | `aGVsbG8=` |
| hex | `hex "AB"` | `4142` |
| fetch | `fetch "https://example.com"` | HTTP GET body |

## License

MIT
