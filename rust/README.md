# shufflr

[![crates.io Shufflr](https://img.shields.io/crates/v/shufflr)](https://crates.io/crates/shufflr)

[![Rust CI](https://github.com/hectortosa/shufflr/actions/workflows/rust-ci-cd.yml/badge.svg?branch=trunk)](https://github.com/hectortosa/shufflr/actions/workflows/shufflr-cargo-ci-cd.yml)

Shuffle lists based on [Fisher-Yates](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle) shuffle algorithm

## Using shufflr

**Shufflr** is available for Rust as crates.io package (also for JavaScript as [NPM package](https://www.npmjs.com/package/shufflr)).

To use **shufflr** in your project simply follow this steps:

Add **shufflr** to your Cargo.toml:

```toml
    [dependencies]
    shufflr = "0.1.0"
```

Import shufflr in your Rust code:

```rust
    use shufflr::shuffle;
```

Use shuffle method with an array to get a copy of it shuffled:

```rust
    let shuffled_array = shuffle(&original_array);
```

## Develop

After cloning the repository, build the project with:

```bash
    cargo build
```

To run **shufflr** tests simply run:

```bash
    cargo test
```
