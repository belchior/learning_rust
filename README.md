# Calc

This project is a math expression calculator and compiler, the reason to it exists is to exercise the syntax, conventions and concepts of the [Rust lang](https://www.rust-lang.org/)

## Development

To run tests:

```shell
cargo test
```

To execute in develop mode

```shell
cargo run '1+(2-3)*((4))'
```

To build:

```shell
cargo build --release
```

To execute:

```shell
target/release/rust_calc '1+(2-3)*((4))'
```
