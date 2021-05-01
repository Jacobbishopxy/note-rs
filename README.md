# Note Rust Lang

A study note of Rust language.

[learning path](./learning-path.md)

## References

Official references:

- [The Rust Programming Language](https://doc.rust-lang.org/book)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/): Unsafe Rust Programing

Other useful references:

- [pretzelhammer/rust-blog](https://github.com/pretzelhammer/rust-blog)
- [rust-unofficial/too-many-lists](https://rust-unofficial.github.io/too-many-lists/index.html)
- [rust-unofficial/patterns](https://rust-unofficial.github.io/patterns/)
- [Possible Rust](https://www.possiblerust.com/)
- [Rust Language Cheat Sheet](https://cheats.rs/)

## Project Structure

```null
├── doc
│   ├── Learn Rust With Entirely Too Many Linked Lists
│   ├── Rust By Example
│   └── The Rust Programming Language
│
├── leetcode
│
├── resources
│
├── trpl
│   ├── Cargo.toml
│   ├── src
│   │   ├── minigrep
│   │   │   ├── data.rs
│   │   │   └── func.rs
│   │   ├── blog.rs
│   │   ├── gui.rs
│   │   ├── hello_macro.rs
│   │   ├── lib.rs
│   │   ├── messenger.rs
│   │   ├── minigrep.rs
│   │   ├── oop_encapsulation.rs
│   │   └── web_server.rs
│   └── trpl_derive
│       ├── src
│       │   └── lib.rs
│       └── Cargo.toml
│
├── trpl_example
│   ├── Cargo.toml
│   └── src
│       ├── bin
│       │   ├── blog.rs
│       │   ├── gui.rs
│       │   ├── hello_macro.rs
│       │   ├── minigrep.rs
│       │   └── web_server.rs
│       └── main.rs
│
├── Cargo.lock
├── Cargo.toml
├── README.md
└── target
```

### leetcode

[README](./leetcode/README.md)

### resources

- [rust.json](./resources/rust.json): VsCode Rust snippets
### trpl [lib]

Examples from 'The Rust Programming Language'.

- minigrep: book TRPL chapter 12
- messenger: book TRPL chapter 15.5
- oop_encapsulation: book TRPL chapter 17.1
- gui: book TRPL chapter 17.2
- blog: book TRPL chapter 17.3
- hello_macro: book TRPL chapter 19.5
- web_server: book TRPL chapter 20

### trpl_example [bin]

- blog run:
  `cargo run -p trpl_example --bin blog`

- minigrep run:
  `cargo run -p trpl_example --bin minigrep -- the poem.txt`
  or
  `CASE_INSENSITIVE=1 cargo run -p trpl_example --bin minigrep -- the poem.txt`

- minigrep test:
  `cargo test -p trpl -- tests_minigrep`

- gui run:
  `cargo run -p trpl_example --bin gui`

- hello_macro run:
  `cargo run -p trpl_example --bin hello_macro`

- web_server run:
  `cargo run -p trpl_example --bin web_server`

### tmll

Examples from 'Learn Rust With Entirely Too Many Linked Lists'.

- first test:
  `cargo test -p tmll -- test_first`

- second test:
  `cargo test -p tmll -- test_second`

- third test:
  `cargo test -p tmll -- test_third`