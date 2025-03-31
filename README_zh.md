# wlin-toolkit

简体中文 | [English](README.md)

[![Package Version](https://img.shields.io/npm/v/wlin-toolkit.svg)](https://www.npmjs.com/package/wlin-toolkit)
[![Downloads](https://img.shields.io/npm/dm/wlin-toolkit.svg)](http://npm-stat.com/charts.html?package=wlin-toolkit&author=&from=&to=)

wlin-toolkit 是一个使用 Rust 编写并编译为 WebAssembly (Wasm) 的 npm 包。它提供了高性能的数据结构操作函数，可以无缝集成到 JavaScript/TypeScript 应用程序中。

## 安装

你可以通过 npm 安装此包：

```sh
npm install wlin-toolkit
```

## 使用

```typescript
import { replaceUndefinedWithNull } from "wlin-toolkit";

// 在不同的数据结构中将 undefined 替换为 null
const array = [1, undefined, 2, undefined, 3];
console.log(replaceUndefinedWithNull(array)); // 输出: [1, null, 2, null, 3]

const object = { a: 1, b: undefined, c: 3 };
console.log(replaceUndefinedWithNull(object)); // 输出: { a: 1, b: null, c: 3 }

const nestedData = {
  a: 1,
  b: undefined,
  c: [1, undefined, 3],
  d: { e: undefined, f: 2 }
};
console.log(replaceUndefinedWithNull(nestedData));
// 输出: { a: 1, b: null, c: [1, null, 3], d: { e: null, f: 2 } }
```

## 开源协议

本项目采用 MIT 协议 - 详情请参阅 [LICENSE](LICENSE) 文件。