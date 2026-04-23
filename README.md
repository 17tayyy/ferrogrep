# ferrogrep

A simple grep clone written in Rust. Built as a learning project to explore Rust concepts like iterators, traits, error handling and CLI tooling.

> ⚠️ Work in progress — built for learning, not production use.

## Features

- Search for patterns in files
- Colored match highlighting
- Case-insensitive search (`-i`)
- Match count (`--count`)
- Read from stdin (pipe support)

## Usage

```bash
# Search in a file
ferrogrep <pattern> <file>

# Case-insensitive
ferrogrep -i <pattern> <file>

# Count matches
ferrogrep --count <pattern> <file>

# Pipe support
cat file.txt | ferrogrep <pattern>
```

## Build

```bash
cargo build --release
```

## Roadmap

- [ ] Recursive directory search
- [ ] Regex support
- [ ] Benchmarks vs native grep
