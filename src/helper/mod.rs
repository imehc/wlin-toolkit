use wasm_bindgen::JsValue;
use web_sys::js_sys::Array;

/// 用于实现JS数组转Rust Vec类型
pub fn js_array_to_vec(js_array: Array) -> Vec<f64> {
  js_array.iter().map(|x| x.as_f64().unwrap()).collect()
}

/// 用于实现转Rust Vec类型转JS数组
pub fn vec_to_js_array(vec: Vec<f64>) -> Array {
  vec.into_iter().map(JsValue::from_f64).collect()
}

pub fn quick_sort(mut arr: &mut [f64]) {
  if arr.len() > 1 {
      let pivot = partition(&mut arr);
      quick_sort(&mut arr[..pivot]);
      quick_sort(&mut arr[pivot + 1..]);
  }
}

pub fn partition(arr: &mut [f64]) -> usize {
  let pivot = arr.len() - 1;
  let mut i = 0;
  for j in 0..pivot {
      if arr[j] < arr[pivot] {
          arr.swap(i, j);
          i += 1;
      }
  }
  arr.swap(i, pivot);
  i
}

pub fn fib_rec(num: i16) -> i64 {
  if num < 2 {
      return 1;
  }
  fib_rec(num - 1) + fib_rec(num - 2)
}
