use web_sys::js_sys::{Array, Object};
use wasm_bindgen::JsValue;

/// 用null替换所有未定义的值
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