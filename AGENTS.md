# qk — Agent Reference

Single binary string toolkit. `qk <command> [args...]`

## Convention

- arg1 is always the context (the thing being operated on)
- stdin replaces arg1 when piped: `echo "hello" | qk upper` = `qk upper "hello"`
- Boolean tools print `true`/`false` and exit 0/1
- Output tools print to stdout with trailing newline
- Exit 2 on error

## Tools

### Boolean (prints true/false, exit 0/1)
```
qk eq <a> <b>                  string equality
qk contains <haystack> <needle> substring check
qk starts <string> <prefix>     prefix check
qk ends <string> <suffix>       suffix check
qk isnum <string>               parseable as number
qk empty <string>               zero-length check
```

### Transform
```
qk trim <string>                strip whitespace
qk upper <string>               UPPERCASE
qk lower <string>               lowercase
qk replace <string> <from> <to> replace all occurrences
qk rev <string>                 reverse
qk slug <string>                slugify (Hello World! -> hello-world)
qk ascii <string>               strip non-ASCII characters
```

### Extract
```
qk between <string> <start> <end>  text between delimiters (first match)
qk sub <string> <offset> <length>  substring (0-indexed, char-based)
qk nth <string> <delim> <index>    nth split element (0-indexed)
qk match <string> <regex>          first regex match (exit 1 if none)
qk matchall <string> <regex>       all regex matches, one per line
```

### Count / Split
```
qk len <string>                 character count
qk count <string> <substring>   occurrence count
qk split <string> <delim>       split to lines
qk chars <string>               one char per line
qk lines <file-or-stdin>        line count
qk uniq <string>                char-level ordered dedupe
```

### Format
```
qk pad <string> <width> [fill]  left-pad (default space)
qk rpad <string> <width> [fill] right-pad (default space)
qk repeat <string> <count>      repeat N times
```

### Encode
```
qk hash <string>                SHA-256 hex
qk b64 <string>                 base64 encode
qk b64 -d <string>              base64 decode
qk hex <string>                 hex encode
qk hex -d <string>              hex decode
qk urlencode <string>           percent-encode for URLs
qk urldecode <string>           decode percent-encoded string
```

### IO
```
qk join <delim>                 join stdin lines with delimiter
qk fetch <url>                  HTTP GET, body to stdout
```

## Pipe Examples

```bash
echo "  Hello  " | qk trim | qk upper              # HELLO
qk split "a,b,c" "," | qk join "-"                  # a-b-c
echo "hello123" | qk match "\d+" | qk len           # 3
qk contains "$input" "error" && echo "found"
qk replace "Hello World" "World" "Rust" | qk slug   # hello-rust
qk urlencode "hello world & more"                    # hello%20world%20%26%20more
echo "café résumé" | qk ascii                        # caf rsum
```
