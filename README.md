# shufflr

[![NPM](https://img.shields.io/npm/v/shufflr)](https://www.npmjs.com/package/shufflr) [![Build Status](https://travis-ci.org/hectortosa/shufflr.svg?branch=master)](https://travis-ci.org/hectortosa/shufflr) ![Dependencies](https://img.shields.io/david/hectortosa/shufflr) ![Dev Dependencies](https://img.shields.io/david/dev/hectortosa/shufflr)

Shuffle lists based on [Fisher-Yates](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle) shuffle algorithm

## Using shufflr

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
