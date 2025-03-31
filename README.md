# wlin-toolkit

[简体中文](README_zh.md) | English

[![Package Version](https://img.shields.io/npm/v/wlin-toolkit.svg)](https://www.npmjs.com/package/wlin-toolkit)
[![Downloads](https://img.shields.io/npm/dm/wlin-toolkit.svg)](http://npm-stat.com/charts.html?package=wlin-toolkit&author=&from=&to=)

wlin-toolkit is a npm package written in Rust and compiled to WebAssembly (Wasm). It provides high-performance data structure manipulation functions that seamlessly integrate with JavaScript/TypeScript applications.

## Installation

You can install this package via npm:

```sh
npm install wlin-toolkit
```

# Usage

``` typescript
import { replaceUndefinedWithNull } from "wlin-toolkit";

// Replace undefined with null in different data structures
const array = [1, undefined, 2, undefined, 3];
console.log(replaceUndefinedWithNull(array)); // Output: [1, null, 2, null, 3]

const object = { a: 1, b: undefined, c: 3 };
console.log(replaceUndefinedWithNull(object)); // Output: { a: 1, b: null, c: 3 }

const nestedData = {
  a: 1,
  b: undefined,
  c: [1, undefined, 3],
  d: { e: undefined, f: 2 }
};
console.log(replaceUndefinedWithNull(nestedData));
// Output: { a: 1, b: null, c: [1, null, 3], d: { e: null, f: 2 } }
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.