# shufflr

[![Build Status](https://travis-ci.org/hectortosa/shufflr.svg?branch=master)](https://travis-ci.org/hectortosa/shufflr)

Shuffle lists based on [Fisher-Yates](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle) shuffle algorithm

## Quick start

To use **shufflr** in your project simply follow this steps:

Install shufflr:

```bash
    npm install shufflr
```

Import shufflr in your JavaScript code:

```javascript
    var shufflr = require('shufflr')
```

Use shuffle method with an array to get a copy of it shuffled:

```javascript
    var shuffledArray = shufflr.shuffle(originalArray)
```