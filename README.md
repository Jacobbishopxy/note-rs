# Note Rust Lang

A study note of Rust language.

- [The Rust Programming Language](https://doc.rust-lang.org/book)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)

## Project Structure

```null
├── doc
│   ├── Rust By Example
│   └── The Rust Programming Language
│
├── minigrep
│   ├── Cargo.toml
│   └── src
│       └── main.rs
│
├── trpl
│   ├── Cargo.toml
│   └── src
│       ├── minigrep
│       │   ├── data.rs
│       │   └── func.rs
│       ├── minigrep.rs
│       └── lib.rs
│
├── Cargo.lock
├── Cargo.toml
├── README.md
└── target
```

### minigrep

minigrep[**bin**]

- run:
  `cargo run -p minigrep -- the poem.txt`
  or
  `CASE_INSENSITIVE=1 cargo run -p minigrep -- the poem.txt`

- test:
  `cargo test -p minigrep`

### trpl

- minigrep[**lib**] (example from 'The Rust Programming Language' chapter 12)
