# Markdown Chunker

A small Rust project created for learning and experimenting with the language.

The project implements a simple two-pass chunker for Markdown files:

1. **Lexing pass** — reads the Markdown input and converts it into tokens.
2. **Parsing/chunking pass** — processes those tokens and groups the document into chunks based on its structure.

The main goal of the project is to practice Rust concepts such as ownership, borrowing, lifetimes, enums, structs, iterators, and parsing text without unnecessary allocations. This is a learning project and the implementation is intentionally simple. It is not intended to be a fully compliant Markdown parser.
