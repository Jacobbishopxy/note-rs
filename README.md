# Note Rust Lang

A study note of Rust language.

1. [learning path](./learning-path.md)
1. [demo projects](https://github.com/Jacobbishopxy/demo)

## References

Official references:

- [The Rust Programming Language](https://doc.rust-lang.org/book)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/): Unsafe Rust Programing
- [Async Book](https://github.com/rust-lang/async-book)

Other useful references:

- [pretzelhammer/rust-blog](https://github.com/pretzelhammer/rust-blog)
- [rust-unofficial/too-many-lists](https://rust-unofficial.github.io/too-many-lists/index.html)
- [rust-unofficial/patterns](https://rust-unofficial.github.io/patterns/)
- [Possible Rust](https://www.possiblerust.com/)
- [Rust Language Cheat Sheet](https://cheats.rs/)
- [rust-algorithms](https://github.com/EbTech/rust-algorithms): Common data structures and algorithms in Rust

## Project Structure

```null
├── doc
│   ├── Asynchronous Programming in Rust
│   ├── Learn Rust With Entirely Too Many Linked Lists
│   ├── Rust By Example
│   ├── Rust Design Patterns
│   ├── The Rust Programming Language
│   └── The Rustonomicon
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

### doc

Study notes/Book translation in Chinese.

- [Asynchronous Programming in Rust](./doc/Asynchronous%20Programming%20in%20Rust)
- [Learn Rust With Entirely Too Many Linked Lists](./doc/Learn%20Rust%20With%20Entirely%20Too%20Many%20Linked%20Lists)
- [Rust By Example](./doc/Rust%20By%20Example)
- [Rust Design Patterns](./doc/Rust%20Design%20Patterns)
- [The Rust Programming Language](./doc/The%20Rust%20Programming%20Language)
- [The Rustonomicon](./doc/The%20Rustonomicon)

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

- fourth test:
  `cargo test -p tmll -- test_fourth`
