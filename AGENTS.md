# qk-str — Agent Reference

String CLI tools. Each binary does one thing. No flags.

## Convention

- arg1 is always the context (the thing being operated on)
- stdin replaces arg1 when piped: `echo "hello" | upper` = `upper "hello"`
- Boolean tools exit 0 (true) or 1 (false), no stdout
- Output tools print to stdout with trailing newline
- Exit 2 on error

## Tools

### Boolean (exit 0 = true, 1 = false)
```
eq <a> <b>                  string equality
contains <haystack> <needle> substring check
starts <string> <prefix>     prefix check
ends <string> <suffix>       suffix check
isnum <string>               parseable as number
empty <string>               zero-length check
```

### Transform
```
trim <string>                strip whitespace
upper <string>               UPPERCASE
lower <string>               lowercase
replace <string> <from> <to> replace all occurrences
rev <string>                 reverse
slug <string>                slugify (Hello World! -> hello-world)
```

### Extract
```
between <string> <start> <end>  text between delimiters (first match)
sub <string> <offset> <length>  substring (0-indexed, char-based)
nth <string> <delim> <index>    nth split element (0-indexed)
match <string> <regex>          first regex match (exit 1 if none)
matchall <string> <regex>       all regex matches, one per line
```

### Count / Split
```
len <string>                 character count
count <string> <substring>   occurrence count
split <string> <delim>       split to lines
chars <string>               one char per line
lines <file-or-stdin>        line count
uniq <string>                char-level ordered dedupe
```

### Format
```
pad <string> <width> [fill]  left-pad (default space)
rpad <string> <width> [fill] right-pad (default space)
repeat <string> <count>      repeat N times
```

### Encode
```
hash <string>                SHA-256 hex
b64 <string>                 base64 encode
b64 -d <string>              base64 decode
hex <string>                 hex encode
hex -d <string>              hex decode
```

### IO
```
join <delim>                 join stdin lines with delimiter
fetch <url>                  HTTP GET, body to stdout
```

## Pipe Examples

```bash
echo "  Hello  " | trim | upper              # HELLO
split "a,b,c" "," | join "-"                  # a-b-c
echo "hello123" | match "\d+" | len           # 3
contains "$input" "error" && echo "found"
replace "Hello World" "World" "Rust" | slug   # hello-rust
```
