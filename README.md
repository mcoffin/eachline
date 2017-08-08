# eachline

`eachline` is a command line utility for piping each line of stdin input to another command.

# Example Usage

```bash
cat some-file-with-concatenated-json.txt | eachline curl -X POST -H 'Content-Type: application/json' http://some-url
```

# Compilation

`eachline` is built with [Cargo](http://crates.io/), so you need only run `cargo build --release` to build the binary.
