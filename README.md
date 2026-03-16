# qk-str

Common string utility functions as a single CLI binary (`qk`), for humans and AI agents.

## Install

```bash
cargo install qk-str
```

Or grab a binary from [GitHub Releases](https://github.com/Necromunger/qk-str/releases).

## How it works

Every command takes **arg1 as the context** (the thing being operated on). If you pipe stdin, it replaces arg1 — that's how composition works:

```bash
echo "  Hello World  " | qk trim | qk upper
# HELLO WORLD

qk split "a,b,c" "," | qk join "-"
# a-b-c

echo "Hello World!" | qk slug
# hello-world

echo "secret" | qk b64 | qk b64 -d
# secret
```

Pipe results straight to your clipboard:

```bash
qk trim "  hello  " | clip
```

Boolean tools print `true`/`false` and exit 0/1 — useful standalone or in `if`/`&&`:

```bash
qk contains "hello world" "world"
# true

qk contains "hello world" "world" && echo "found it"
qk isnum "42.5" && echo "it's a number"
```

## Tools

| Tool | Example | Output |
|------|---------|--------|
| eq | `qk eq "a" "b"` | `true`/`false` |
| contains | `qk contains "haystack" "needle"` | `true`/`false` |
| starts | `qk starts "hello" "hel"` | `true`/`false` |
| ends | `qk ends "file.txt" ".txt"` | `true`/`false` |
| isnum | `qk isnum "42.5"` | `true`/`false` |
| empty | `qk empty ""` | `true`/`false` |
| len | `qk len "hello"` | `5` |
| trim | `qk trim "  hi  "` | `hi` |
| upper | `qk upper "hello"` | `HELLO` |
| lower | `qk lower "Hello"` | `hello` |
| replace | `qk replace "foo bar" "foo" "baz"` | `baz bar` |
| between | `qk between "a[b]c" "[" "]"` | `b` |
| split | `qk split "a,b,c" ","` | `a\nb\nc` |
| count | `qk count "abab" "ab"` | `2` |
| rev | `qk rev "hello"` | `olleh` |
| repeat | `qk repeat "ab" 3` | `ababab` |
| pad | `qk pad "42" 5 "0"` | `00042` |
| rpad | `qk rpad "hi" 5 "."` | `hi...` |
| sub | `qk sub "hello world" 6 5` | `world` |
| lines | `qk lines file.txt` | line count |
| nth | `qk nth "a,b,c" "," 1` | `b` |
| join | `... \| qk join ","` | joins stdin lines |
| uniq | `qk uniq "abcabc"` | `abc` |
| chars | `qk chars "hello"` | `h\ne\nl\nl\no` |
| match | `qk match "hello123" "\d+"` | `123` |
| matchall | `qk matchall "a1b2" "\d"` | `1\n2` |
| slug | `qk slug "Hello World!"` | `hello-world` |
| hash | `qk hash "data"` | SHA-256 hex |
| b64 | `qk b64 "hello"` | `aGVsbG8=` |
| hex | `qk hex "AB"` | `4142` |
| fetch | `qk fetch "https://example.com"` | HTTP GET body |
| urlencode | `qk urlencode "hello world"` | `hello%20world` |
| urldecode | `qk urldecode "hello%20world"` | `hello world` |
| ascii | `qk ascii "café"` | `caf` |

## License

MIT
