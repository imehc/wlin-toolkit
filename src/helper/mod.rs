use wasm_bindgen::JsValue;
use web_sys::js_sys::{Array, Object};

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

pub fn replace_undefined_with_null(value: JsValue) -> JsValue {
    if value.is_undefined() {
        return JsValue::NULL;
    }

    if value.is_null() || !value.is_object() {
        return value;
    }

    if Array::is_array(&value) {
        let array = Array::from(&value);
        return array
            .iter()
            .map(|item| replace_undefined_with_null(item))
            .collect::<Array>()
            .into();
    }

    if let Some(obj) = Object::try_from(&value) {
        let new_obj = Object::new();
        let entries = Object::entries(&obj);
        for entry in entries.iter() {
            let entry = Array::from(&entry);
            let key = entry.get(0);
            let val = entry.get(1);
            let processed_val = replace_undefined_with_null(val);
            web_sys::js_sys::Reflect::set(&new_obj, &key, &processed_val).unwrap();
        }
        return new_obj.into();
    }

    value
}
