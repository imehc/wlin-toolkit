# UI 项目重构说明

## 📁 最终目录结构

```
ui/
├── components/              # UI 组件（React & Svelte）
│   ├── ReplaceUndefined.tsx      # React 组件 - 替换 undefined
│   ├── DigitUppercase.svelte     # Svelte 组件 - 数字转大写（使用 runes）
│   └── DocSidebar.astro          # 文档侧边栏
│
├── layouts/                 # 布局组件
│   ├── Layout.astro              # 基础布局
│   └── DocLayout.astro           # 文档布局（带侧边栏）
│
├── lib/                     # 工具库
│   └── apiCategories.ts          # API 分类配置
│
├── pages/                   # 页面路由
│   ├── index.astro               # 首页
│   └── docs/                     # 文档页面（MDX格式）
│       ├── data-processing/      # 数据处理分类
│       │   └── replace-undefined.mdx
│       └── text-conversion/      # 文本转换分类
│           └── digit-uppercase.mdx
│
├── public/                  # 静态资源
└── styles/                  # 全局样式
    └── global.css
```

## ✨ 重构要点

### 1. **组件命名规范**
- ✅ 移除 `Demo` 后缀
- ✅ 使用清晰的功能名称：`ReplaceUndefined`、`DigitUppercase`

### 2. **文档格式：MDX**
- ✅ 文档内容使用 MDX 编写
- ✅ 交互组件通过 import 引入
- ✅ 内容和交互完全分离

示例：
```mdx
---
layout: ~/layouts/DocLayout.astro
title: "API 名称"
---

import MyComponent from "~/components/MyComponent.tsx";

## 文档内容

<MyComponent client:load />

更多文档内容...
```

### 3. **组件技术栈**
- **React 组件**: `ReplaceUndefined.tsx`
  - 使用 React Hooks
  - 适合复杂交互逻辑

- **Svelte 组件**: `DigitUppercase.svelte`
  - 使用 Svelte 5 Runes 语法 (`$state`, `$derived`)
  - 更简洁的响应式代码

### 4. **布局统一**
- ✅ 统一间距：使用 `py-1`, `mb-3`, `gap-4` 等
- ✅ 紧凑设计：减小 padding 和 margin
- ✅ 响应式：支持移动端和桌面端

### 5. **移除的文件**
```
❌ ui/pages/demo/                    # 旧的 demo 目录
❌ ui/components/CodeEditor.astro     # 不再使用的组件
❌ ui/components/CodeOutput.astro
❌ ui/components/ExampleButtons.astro
❌ ui/components/PerformanceMetrics.astro
❌ ui/layouts/ApiDemo.astro           # 旧布局
❌ ui/lib/performance.ts              # 不再需要性能计时
❌ *.astro 格式的文档页面             # 改用 MDX
```

## 🎯 开发流程

### 添加新的 API 文档

1. **在 `lib/apiCategories.ts` 中添加配置**
```typescript
{
  id: "new-api",
  name: "新功能",
  description: "功能描述",
  path: "/docs/category/new-api",
  category: "category-id"
}
```

2. **创建 React 或 Svelte 组件**
```bash
# React 组件
ui/components/NewFeature.tsx

# 或 Svelte 组件
ui/components/NewFeature.svelte
```

3. **创建 MDX 文档**
```bash
ui/pages/docs/category/new-api.mdx
```

示例 MDX：
```mdx
---
layout: ~/layouts/DocLayout.astro
title: "新功能 - WASM Toolkit"
apiName: "新功能"
description: "功能描述"
currentPath: "/docs/category/new-api"
---

import NewFeature from "~/components/NewFeature.tsx";

## 功能说明

功能介绍...

<NewFeature client:load />

## API 参考

API 文档...
```

## 🛠️ 技术栈

- **Astro**: 静态站点生成 + MDX 支持
- **React**: 交互组件
- **Svelte 5**: 交互组件（使用 Runes）
- **daisyUI**: UI 组件库
- **Tailwind CSS 4**: 样式框架
- **TypeScript**: 类型安全
- **WASM**: Rust 编译模块

## 📦 依赖包

```json
{
  "devDependencies": {
    "@astrojs/mdx": "^x.x.x",
    "@astrojs/react": "^x.x.x",
    "@astrojs/svelte": "^x.x.x",
    "astro": "^5.x.x",
    "daisyui": "^5.x.x",
    "react": "^18.x.x",
    "svelte": "^5.x.x",
    "tailwindcss": "^4.x.x"
  }
}
```

## 🎨 设计系统

### 间距规范
- 小间距: `gap-2`, `py-1`, `mb-2`
- 中间距: `gap-3`, `py-2`, `mb-3`
- 大间距: `gap-4`, `py-3`, `mb-4`

### 组件尺寸
- 按钮: `btn-sm`, `btn-xs`
- 输入框: `input-sm`
- 字体: `text-sm`, `text-xs`, `text-lg`

### 颜色方案
- 主色: `primary`, `secondary`
- 状态: `success`, `error`, `info`
- 背景: `base-100`, `base-200`, `base-300`

## 🚀 命令

```bash
# 开发
npm run dev

# 构建
npm run build

# 预览
npm run preview
```
