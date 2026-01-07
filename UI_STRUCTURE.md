# UI 项目结构说明

## 📁 目录结构

```
ui/
├── components/          # 可复用 UI 组件
│   ├── CodeEditor.astro          # 代码编辑器组件
│   ├── CodeOutput.astro          # 代码输出组件
│   ├── PerformanceMetrics.astro  # 性能指标展示组件
│   ├── ExampleButtons.astro      # 示例按钮组件
│   └── DocSidebar.astro          # 文档侧边栏组件
│
├── layouts/             # 布局组件
│   ├── Layout.astro              # 基础布局
│   └── DocLayout.astro           # 文档布局（带侧边栏导航）
│
├── lib/                 # 可复用逻辑
│   ├── apiCategories.ts          # API 分类配置
│   └── performance.ts            # 性能监测工具
│
├── pages/               # 页面路由
│   ├── index.astro               # 首页
│   └── docs/                     # 文档页面
│       ├── data-processing/      # 数据处理类
│       │   └── replace-undefined.astro
│       └── text-conversion/      # 文本转换类
│           └── digit-uppercase.astro
│
├── public/              # 静态资源
└── styles/              # 全局样式
    └── global.css
```

## 🎨 设计原则

### 1. UI 和逻辑分离

- **UI 组件** (`components/`): 纯展示组件，接收 props 并渲染
- **逻辑工具** (`lib/`): 可复用的业务逻辑和工具函数
- **页面** (`pages/`): 组合组件和逻辑，处理交互

### 2. 基于分类的文档结构

不再使用 `/demo/*` 路径，而是按功能分类：
- `/docs/data-processing/*` - 数据处理相关 API
- `/docs/text-conversion/*` - 文本转换相关 API

### 3. 使用 daisyUI 组件库

所有 UI 组件都使用 daisyUI 的样式系统：
- `btn`, `card`, `alert`, `badge` 等组件
- 响应式设计
- 主题支持

## 🧩 组件说明

### CodeEditor.astro
```astro
<CodeEditor
  id="my-editor"
  label="输入代码"
  defaultValue="..."
  placeholder="..."
  rows={8}
/>
```

### CodeOutput.astro
```astro
<CodeOutput
  id="my-output"
  label="输出结果"
  defaultValue="..."
/>
```

### PerformanceMetrics.astro
```astro
<PerformanceMetrics
  metrics={[
    { id: "time", label: "处理时间", color: "primary" },
    { id: "count", label: "字符数", color: "info" }
  ]}
/>
```

### ExampleButtons.astro
```astro
<ExampleButtons
  examples={[
    { id: "example1", label: "示例 1", description: "描述" }
  ]}
/>
```

### DocSidebar.astro
```astro
<DocSidebar currentPath="/docs/data-processing/replace-undefined" />
```

## 🔧 工具函数

### lib/performance.ts

```typescript
import { PerformanceTimer, formatDuration } from "~/lib/performance";

const timer = new PerformanceTimer();
timer.start();
// ... 执行操作
const result = timer.end();
console.log(formatDuration(result.duration)); // "1.23 ms"
```

### lib/apiCategories.ts

```typescript
import { apiCategories, getAllApis, getApiById } from "~/lib/apiCategories";

// 获取所有分类
const categories = apiCategories;

// 获取所有 API
const apis = getAllApis();

// 根据 ID 获取 API
const api = getApiById("replace-undefined");
```

## 📝 添加新的 API 文档

### 1. 在 `lib/apiCategories.ts` 中添加配置

```typescript
{
  id: "new-api",
  name: "新 API",
  description: "API 描述",
  path: "/docs/category/new-api",
  category: "category-id"
}
```

### 2. 创建页面文件

```
ui/pages/docs/[category]/[api-name].astro
```

### 3. 使用 DocLayout 和可复用组件

```astro
---
import DocLayout from "~/layouts/DocLayout.astro";
import CodeEditor from "~/components/CodeEditor.astro";
// ...
---

<DocLayout
  title="..."
  apiName="..."
  description="..."
  currentPath="/docs/category/api-name"
>
  <!-- 内容 -->
</DocLayout>
```

## 🎯 技术栈

- **Astro**: 静态站点生成
- **daisyUI**: UI 组件库
- **Tailwind CSS 4**: 样式框架
- **TypeScript**: 类型安全
- **WebAssembly**: Rust 编译的 WASM 模块

## 🚀 开发命令

```bash
# 开发模式
npm run dev

# 构建
npm run build

# 预览
npm run preview
```
