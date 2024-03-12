async function main() {
  const { helloWorld, fib, sendArrayToJs, sendObjToJs, testPoint, fibRust, quickSort } = await import('../pkg/index');
  helloWorld();
  console.log(fib(30));
  console.log(sendArrayToJs());
  console.log(sendObjToJs());
  console.log(testPoint());
  console.log("======================🌃======================")
  let fibDuration = { JS: 0, Rust: 0 };
  let quickSortDuration = { JS: 0, Rust: 0 };

  // 测试JS 斐波那契耗时
  const jsStart1 = performance.now();
  fibJavascript(32);
  fibDuration.JS = (performance.now() - jsStart1).toFixed(0);
  // 测试JS 快排耗时
  const jsStart2 = performance.now();
  jsQuickSort(array)
  quickSortDuration.JS = (performance.now() - jsStart2).toFixed(0);

  // 测试Rust 斐波那契耗时
  const rustStart1 = performance.now();
  fibRust(32);
  fibDuration.Rust = (performance.now() - rustStart1).toFixed(0);
  // 测试Rust 快排耗时
  const rustStart2 = performance.now();
  quickSort(array)
  quickSortDuration.Rust = (performance.now() - rustStart2).toFixed(0);

  console.log(`斐波那契数列 - Wasm耗时为：${fibDuration.Rust}`)
  console.log(`斐波那契数列 - JS耗时为：${fibDuration.JS}`)
  console.log(`快速排序 - Wasm耗时为：${quickSortDuration.Rust}`)
  console.log(`快速排序 - JS耗时为：${quickSortDuration.JS}`)
}

function fibJavascript(n) {
  if (n < 1) return 0;
  if (n <= 2) return 1;
  return fibJavascript(n - 1) + fibJavascript(n - 2)
}

const jsQuickSort = function (arr) {
  // 递归结束条件
  if (arr.length < 2) return arr;

  // 基准
  const pivot = arr.splice(0, 1);
  // 左区
  const left = [];
  // 右区
  const right = [];

  // 将剩余元素按照一定规则，分配到左区、右区。
  for (let i = 0; i < arr.length; i++) {
    // 大于基准值的分配到右区，小于基准值的分配到左区
    if (arr[i] > pivot[0]) {
      right.push(arr[i])
    } else {
      left.push(arr[i])
    }
  }

  // 返回 左区 拼 基准 拼 右区， 再对左区、右区分别重选基准分区
  return jsQuickSort(left).concat(pivot).concat(jsQuickSort(right));
}

let array = [];
for (let i = 0; i < 1000000; i++) {
  array.push(Math.floor(Math.random() * 100000));
}


main();
