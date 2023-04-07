# shufflr

[![NPM](https://img.shields.io/npm/v/shufflr)](https://www.npmjs.com/package/shufflr) ![Shufflr CI/CD](https://github.com/hectortosa/shufflr/workflows/Shufflr%20CI/CD/badge.svg)

Shuffle lists based on [Fisher-Yates](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle) shuffle algorithm

## Using shufflr

**Shufflr** is available for Rust as crates.io package and for JavaScript as NPM package.

### Rust

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

### JavaScript

To use **shufflr** in your project simply follow this steps:

Install **shufflr**:

```bash
    npm install shufflr
```

Import shufflr in your JavaScript code:

```javascript
    var shufflr = require('shufflr');
```

Or:

```javascript
    import { shuffle } from 'shufflr';
```

Use shuffle method with an array to get a copy of it shuffled:

```javascript
    var shuffledArray = shufflr.shuffle(originalArray)
```

Or:

```javascript
    var shuffledArray = shuffle(originalArray)
```

## Develop

After cloning the repository, install the dev dependencies (**shufflr** does not have runtime dependencies):

```bash
    npm install
```

To run **shufflr** tests simply run:

```bash
    npm run test
```
