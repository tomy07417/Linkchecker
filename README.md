# Rust Linkchecker

A concurrent link checker written in Rust that reads a Markdown file containing URLs, validates them via HTTP requests, extracts page titles when possible, and generates a cleaned Markdown output.

---

## 📌 Project Description

This project implements a command-line tool that:

- Takes a Markdown file as input
- Extracts all URLs found in the file
- Processes URLs concurrently (up to **32 at a time**)
- For each URL:
  - Performs an HTTP request
  - If successful:
    - Parses the HTML
    - Extracts the `<title>` tag
  - If unsuccessful:
    - Converts the error into a human-readable message
- Outputs a new Markdown file with properly formatted links

---

## ✨ Example

### Input (`input.md`)
```md
- https://www.rust-lang.org
- [Search](https://www.google.com)
- https://this-link-does-not-exist.xyz
```

### Output (`output.md`)

```
[The Rust Programming Language](https://www.rust-lang.org)
[Google](https://www.google.com)
[Not Found](https://this-link-does-not-exist.xyz)
```

---

## 🚀 Usage

```bash
cargo run -- data/input.md data/output.md
```

Or with the Makefile:

```bash
make run input=data/input.md output=data/output.md
```

---

## 🧾 Input Format

The input file can be any Markdown (or plain text) content. The parser scans for
URLs that start with `http://` or `https://` anywhere in the text and keeps the
first occurrence of each URL (duplicates are ignored).

Example (one URL per line works great too):

```md
https://www.rust-lang.org/
https://www.mozilla.org/
https://www.kernel.org/
```
