<script lang="ts">
  import { onMount } from "svelte";

  let input = $state("123.45");
  let result = $state("");
  let error = $state(false);
  let charCount = $state(0);
  let digitUppercase: ((n: number) => string) | null = $state(null);
  let loading = $state(true);

  const examples: Record<string, string> = {
    zero: "0",
    hundred: "100",
    thousand: "1000",
    decimal: "123.45",
    large: "999999"
  };

  onMount(async () => {
    try {
      const wasm = await import("#/wlin_toolkit");
      digitUppercase = wasm.digitUppercase;
      loading = false;
      convertNumber();
    } catch (err) {
      result = `WASM 模块加载失败: ${(err as Error).message}`;
      error = true;
      loading = false;
    }
  });

  function convertNumber() {
    if (!digitUppercase) return;

    try {
      const number = parseFloat(input);
      if (isNaN(number)) {
        result = "请输入有效的数字";
        error = true;
        return;
      }

      const converted = digitUppercase(number);
      result = converted;
      charCount = converted.length;
      error = false;
    } catch (err) {
      result = `错误: ${(err as Error).message}`;
      error = true;
      charCount = 0;
    }
  }

  function loadExample(key: string) {
    input = examples[key];
    convertNumber();
  }

  function randomInt() {
    input = Math.floor(Math.random() * 100000).toString();
    convertNumber();
  }

  function randomDecimal() {
    input = (Math.random() * 1000).toFixed(2);
    convertNumber();
  }

  function randomLarge() {
    input = Math.floor(Math.random() * 999999999).toString();
    convertNumber();
  }

  function handleKeyPress(e: KeyboardEvent) {
    if (e.key === "Enter") {
      convertNumber();
    }
  }
</script>

<div class="not-prose card bg-base-100 shadow-xl my-8">
  <div class="card-body">
    <h2 class="card-title text-xl mb-4">在线演示</h2>

    <div class="grid lg:grid-cols-2 gap-6">
      <!-- 左侧：输入 -->
      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="digit-input">
            <span class="label-text font-medium text-sm">输入数字</span>
          </label>
          <input
            type="number"
            id="digit-input"
            class="input input-bordered w-full"
            placeholder="例如: 123.45"
            bind:value={input}
            oninput={convertNumber}
            onkeypress={handleKeyPress}
            step="any"
          />
        </div>

        <button class="btn btn-primary btn-block" onclick={convertNumber}>
          🔤 转换为大写
        </button>

        <div class="form-control">
          <label class="label" for="examples">
            <span class="label-text font-medium text-sm">快速示例</span>
          </label>
          <div id="examples" class="flex flex-wrap gap-2">
            <button class="btn btn-sm btn-outline" onclick={() => loadExample("zero")}>0</button>
            <button class="btn btn-sm btn-outline" onclick={() => loadExample("hundred")}>100</button>
            <button class="btn btn-sm btn-outline" onclick={() => loadExample("thousand")}>1000</button>
            <button class="btn btn-sm btn-outline" onclick={() => loadExample("decimal")}>123.45</button>
            <button class="btn btn-sm btn-outline" onclick={() => loadExample("large")}>999999</button>
          </div>
        </div>

        <div class="form-control">
          <label class="label" for="random">
            <span class="label-text font-medium text-sm">随机测试</span>
          </label>
          <div id="random" class="grid grid-cols-3 gap-2">
            <button class="btn btn-sm btn-outline" onclick={randomInt}>
              整数
            </button>
            <button class="btn btn-sm btn-outline" onclick={randomDecimal}>
              小数
            </button>
            <button class="btn btn-sm btn-outline" onclick={randomLarge}>
              大数
            </button>
          </div>
        </div>
      </div>

      <!-- 右侧：输出 -->
      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="result">
            <span class="label-text font-medium text-sm">转换结果</span>
          </label>
          <div id="result" class="alert shadow-lg h-32 flex items-center justify-center bg-base-200">
            {#if loading}
              <div class="text-sm text-base-content/60">加载中...</div>
            {:else}
              <div class="text-center">
                <div class="text-2xl font-bold mb-2" class:text-primary={!error} class:text-error={error}>
                  {result}
                </div>
                <div class="text-xs text-base-content/60">中文大写数字</div>
              </div>
            {/if}
          </div>
        </div>

        <div class="stats shadow w-full bg-base-200">
          <div class="stat py-3">
            <div class="stat-title text-xs">字符长度</div>
            <div class="stat-value text-2xl text-primary">{charCount}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
