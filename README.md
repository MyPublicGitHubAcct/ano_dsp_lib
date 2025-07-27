# Ano DSP Library

This is a DSP libary by Anoesis Audio, written in [Rust](https://www.rust-lang.org), and intended to run on embedded and desktop.

## More to come here as the library is built

words

## Compiling and Testing Information

Use the command below to start the new library. This will create a _lib.rs_ file and no _main.rs_.

```zsh
cargo new ano_dsp_lib --lib
```

To check the code prior to compiling, use ```cargo check```.

Since this is a library, there is nothing to run. Using ```cargo test``` rather than ```cargo run``` will work.

To build in no-std mode, there cannot be a _main.rs_ file in the src directory. Instead, and _examples_ directory will be used & running that will be done as shown. The three commands that are most useful are:

```zsh
cargo check
cargo test
cargo build --no-default-features
cargo run --example demo
cargo clean
```
