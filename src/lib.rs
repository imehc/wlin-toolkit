use helper::{fib_rec, js_array_to_vec, quick_sort, vec_to_js_array};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{console, js_sys::Array};

mod helper;

// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = helloWorld)]
pub fn hello_world() {
    console::log_1(&JsValue::from_str("Hello World!"));
}

/// 斐波那契数列，时间复杂度 O(2^n)
#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

/// 返回数组
#[wasm_bindgen(js_name = sendArrayToJs)]
pub fn send_array_to_js() -> Box<[JsValue]> {
    vec![
        JsValue::NULL,
        JsValue::UNDEFINED,
        JsValue::from_str("123"),
        JsValue::TRUE,
        JsValue::FALSE,
    ]
    .into_boxed_slice()
}

#[derive(Serialize, Deserialize)]
pub struct Obj {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<i32>>,
    pub field3: [f32; 4],
    pub field4: bool,
    pub field5: String,
}

/// 返回对象
#[wasm_bindgen(js_name = sendObjToJs)]
pub fn send_obj_to_js() -> JsValue {
    let mut map: HashMap<u32, String> = HashMap::new();
    map.insert(0, String::from("ex"));

    let obj = Obj {
        field1: map,
        field2: vec![vec![1, 2], vec![3, 4]],
        field3: [1., 2., 3., 4.],
        field4: true,
        field5: String::from("字符串"),
    };

    serde_wasm_bindgen::to_value(&obj).unwrap()
}

#[wasm_bindgen(module = "/js2rust/point.js")]
extern "C" {
    pub type Point;

    #[wasm_bindgen(constructor)]
    fn new(x: i32, y: i32) -> Point;

    #[wasm_bindgen(method, getter)]
    fn get_x(this: &Point) -> i32;

    #[wasm_bindgen(method, getter)]
    fn get_y(this: &Point) -> i32;

    #[wasm_bindgen(method, setter)]
    fn set_x(this: &Point, x: i32) -> i32;

    #[wasm_bindgen(method, setter)]
    fn set_y(this: &Point, y: i32) -> i32;

    #[wasm_bindgen(method)]
    fn add(this: &Point, p: Point);
}

/// 这个函数 JS 侧可以继续进行调用，最终会返回一个 point 对象实例
#[wasm_bindgen(js_name = testPoint)]
pub fn test_point() -> Point {
    let p: Point = Point::new(10, 10);
    let p1: Point = Point::new(6, 3);
    p.add(p1);
    p
}

/// 快速排序
#[wasm_bindgen(js_name = quickSort)]
pub fn quick_sort_js(arr: Array) -> Array {
    let mut vec = js_array_to_vec(arr);
    quick_sort(&mut vec);
    vec_to_js_array(vec)
}

/// 斐波那契数列
#[wasm_bindgen(js_name = fibRust)]
pub fn fib_recursion(time: i16) -> i64 {
    let mut result = 0;
    let mut i = 1;
    while i < time {
        result = fib_rec(i);
        i += 1;
    }
    result
}
