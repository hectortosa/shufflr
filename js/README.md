# shufflr

[![NPM Shufflr](https://img.shields.io/npm/v/shufflr)](https://www.npmjs.com/package/shufflr)

[![JavaScript CI](https://github.com/hectortosa/shufflr/actions/workflows/js-ci-cd.yml/badge.svg?branch=trunk)](https://github.com/hectortosa/shufflr/actions/workflows/shufflr_ci-cd.yml)

Shuffle lists based on [Fisher-Yates](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle) shuffle algorithm

## Using shufflr

> Note: **Shufflr** is also available for Rust as [crates.io package](https://crates.io/crates/shufflr).

To use **shufflr** in your project simply follow this steps:

Install **shufflr** (trying to cover all package manager nowadays):

```bash
    npm i shufflr
```

Or:

```bash
    pnpm add shufflr
```

Or:

```bash
    yarn add shufflr
```

Or:

```bash
    bun add shufflr
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

## :heart: Like the project?

If you like this project (or [any other](https://github.com/hectortosa)) and want to help me contiue to improve it or create new ones, check my Ko-fi profile and consider buying me a Speciality Coffee:

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H3P6FO7)