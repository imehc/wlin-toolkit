async function main() {
  const { replaceUndefinedWithNull, digitUppercase } = await import('../pkg/index');
  // 测试replaceUndefinedWithNull函数
  const testArray = [1, undefined, 2, undefined, 3];
  const resultArray = replaceUndefinedWithNull(testArray);
  console.log('replaceUndefinedWithNull result:', resultArray);
  // 测试混合数据
  const testMixedArray = {
    a: 1,
    b: undefined,
    c: [1, undefined, 3],
    d: [],
    e: true,
    f: {},
    g: { h: 'i' },
    j: "2023-07-13T12:00:00.000Z",
    k: null,
  }
  const resultMixedArray = replaceUndefinedWithNull(testMixedArray);
  console.log('replaceUndefinedWithNull mixed result:', resultMixedArray);
  // 测试对象中带undefined
  const testObject = { a: 1, b: undefined, c: 3 };
  const resultObject = replaceUndefinedWithNull(testObject);
  console.log('replaceUndefinedWithNull object result:', resultObject);
  // 测试基本变量
  const testNumber = 42;
  const resultNumber = replaceUndefinedWithNull(testNumber);
  console.log('replaceUndefinedWithNull number result:', resultNumber);

  const testString = "Hello";
  const resultString = replaceUndefinedWithNull(testString);
  console.log('replaceUndefinedWithNull string result:', resultString);

  const testBoolean = true;
  const resultBoolean = replaceUndefinedWithNull(testBoolean);
  console.log('replaceUndefinedWithNull boolean result:', resultBoolean);

  // 测试数字转换为大写
  console.log("🚀 ~ index.js:3 ~ main ~ digitUppercase:", digitUppercase(123.45))
  console.log("🚀 ~ index.js:3 ~ main ~ digitUppercase:", digitUppercase(0))
  console.log("🚀 ~ index.js:3 ~ main ~ digitUppercase:", digitUppercase(100))
  console.log("🚀 ~ index.js:3 ~ main ~ digitUppercase:", digitUppercase(10011))
}


main();
