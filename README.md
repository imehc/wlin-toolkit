# wlin-rs-wasm

[![Package Version](https://img.shields.io/npm/v/wlin-rs-wasm.svg)](https://www.npmjs.com/package/wlin-rs-wasm)
[![Downloads](https://img.shields.io/npm/dm/wlin-rs-wasm.svg)](http://npm-stat.com/charts.html?package=wlin-rs-wasm&author=&from=&to=)

wlin-rs-wasm is a npm package written in Rust and compiled to WebAssembly (Wasm).

## Installation

You can install this package via npm:

```sh
npm install wlin-rs-wasm
```

# Usage

``` typescript
import { fib } from "wlin-rs-wasm";

// Calling Rust function
const result = fib(30);
console.log(result); // Output: 832040

```

# Contributing

Contributions are welcome! You can find this project on GitHub: [wlin-rs-wasm](https://github.com/Richt20/rs-wasm).