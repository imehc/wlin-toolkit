import { useState } from "react";

export default function ReplaceUndefinedDemo() {
  const [input, setInput] = useState(`{
  "name": "John",
  "age": undefined,
  "hobbies": ["reading", undefined],
  "email": undefined
}`);
  const [output, setOutput] = useState<string>("点击按钮查看结果...");
  const [error, setError] = useState(false);

  const examples = {
    simple: `{
  "name": "John",
  "age": undefined,
  "active": true,
  "score": null
}`,
    array: `[
  1,
  undefined,
  "hello",
  null,
  undefined
]`,
    nested: `{
  "user": {
    "name": "Alice",
    "bio": undefined,
    "settings": {
      "theme": "dark",
      "notifications": undefined
    }
  },
  "posts": [
    {
      "title": "Hello World",
      "content": undefined
    }
  ]
}`
  };

  const processData = async () => {
    try {
      const wasm = await import("#/wlin_toolkit");
      const { replaceUndefinedWithNull } = wasm;

      const inputText = input.trim();
      if (!inputText) {
        setOutput("请输入要处理的数据");
        setError(true);
        return;
      }

      const testData = Function(`"use strict"; return (${inputText})`)();
      const result = replaceUndefinedWithNull(testData);

      setOutput(JSON.stringify(result, null, 2));
      setError(false);
    } catch (err) {
      setOutput(`错误: ${(err as Error).message}`);
      setError(true);
    }
  };

  const loadExample = (exampleKey: keyof typeof examples) => {
    setInput(examples[exampleKey]);
    setTimeout(processData, 100);
  };

  return (
    <div className="not-prose card bg-base-100 shadow-xl my-8">
      <div className="card-body">
        <h2 className="card-title text-xl mb-4">在线演示</h2>

        <div className="grid lg:grid-cols-2 gap-6">
          {/* 左侧：输入 */}
          <div className="space-y-4">
            <div className="form-control">
              <label className="label">
                <span className="label-text font-medium text-sm">输入数据</span>
              </label>
              <textarea
                className="textarea textarea-bordered font-mono text-sm h-64 leading-relaxed"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                placeholder="输入包含 undefined 的 JavaScript 对象..."
              />
            </div>

            <button
              className="btn btn-primary btn-block"
              onClick={processData}
            >
              🔄 转换数据
            </button>

            <div className="form-control">
              <label className="label">
                <span className="label-text font-medium text-sm">快速示例</span>
              </label>
              <div className="flex flex-wrap gap-2">
                <button
                  className="btn btn-sm btn-outline"
                  onClick={() => loadExample("simple")}
                >
                  简单对象
                </button>
                <button
                  className="btn btn-sm btn-outline"
                  onClick={() => loadExample("array")}
                >
                  数组处理
                </button>
                <button
                  className="btn btn-sm btn-outline"
                  onClick={() => loadExample("nested")}
                >
                  嵌套结构
                </button>
              </div>
            </div>
          </div>

          {/* 右侧：输出 */}
          <div className="space-y-4">
            <div className="form-control">
              <label className="label">
                <span className="label-text font-medium text-sm">转换结果</span>
              </label>
              <div className="mockup-code bg-base-200 h-64 overflow-auto">
                <pre className="px-4 py-2"><code className={error ? "text-error" : "text-success"}>{output}</code></pre>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
