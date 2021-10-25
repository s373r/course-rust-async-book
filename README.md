# Course: Asynchronous Programming in Rust

Course link: https://rust-lang.github.io/async-book/

Status: 🚧 `[work in progress]`

### Index legend

- 📝 - a link to a book page
- ✏️ - a link to an `.rs` file (code)

## Index

- [📝 1. Getting Started](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)
  - [📝 1.1. Why Async?](https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html)
  - [📝 1.2. The State of Asynchronous Rust](https://rust-lang.github.io/async-book/01_getting_started/03_state_of_async_rust.html)
  - [✏️ 1.3. async/.await Primer](01_getting_started/src/main.rs)
- [📝 2. Under the Hood: Executing Futures and Tasks](https://rust-lang.github.io/async-book/02_execution/01_chapter.html)
  - [📝 2.1. The Future Trait](https://rust-lang.github.io/async-book/02_execution/02_future.html) 
  - [✏️ 2.2. Task Wakeups with Waker](02_execution/src/main.rs#L9)
  - [✏️ 2.3. Applied: Build an Executor](02_execution/src/main.rs#L90)
  - [📝 2.4. Executors and System IO](https://rust-lang.github.io/async-book/02_execution/05_io.html)

## Notes

### Comments

- Some of my thoughts are prefixed with `NOTE:`
  - Example: `// NOTE: Algorithm complexity: O(n)`
- Resolved course TODOs are prefixed with `DONE:`
  - Example: `// DONE: ^ Uncomment the above 2 lines to see the compiler error`
- Other comments copied from the course
                                        
### A new chapter

> ℹ️ Cargo projects cannot be named leading from a digit

To create a new chapter-related subfolder, please use the following format: `cargo new N_name --name _N_name` 

#### Quick commands

> ℹ️ Update N and NAME variable values

Unix-like:
```shell
N=01; NAME=getting_started; cargo new "${N}_${NAME}" --name "_${N}_${NAME}"
```

Windows (Powershell):
```shell
$N='01'; $NAME='getting_started'; cargo new ${N}_${NAME} --name _${N}_${NAME}
```

## Code conduction

This project uses [Gitmoji](https://gitmoji.carloscuesta.me) for commit messages.

## License

[GPLv3+](LICENSE)
