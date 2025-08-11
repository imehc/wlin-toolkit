use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};
use web_sys::js_sys::Number;

use crate::helper::{digit_uppercase, replace_undefined_with_null};

mod helper;

// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// 替换 undefined 为 null
#[wasm_bindgen(js_name = replaceUndefinedWithNull)]
pub fn replace_undefined_with_null_js(value: JsValue) -> JsValue {
    replace_undefined_with_null(value)
}

/// 数字转中文大写
#[wasm_bindgen(js_name = digitUppercase)]
pub fn digit_uppercase_js(n: Number) -> Result<String, JsError> {
    digit_uppercase(n.value_of()).map_err(|e| JsError::new(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_uppercase() {
        assert_eq!(digit_uppercase(0.0).unwrap(), "零元整");
        assert_eq!(digit_uppercase(66.0).unwrap(), "陆拾陆元整");
        assert_eq!(digit_uppercase(10011.0).unwrap(), "壹万零壹拾壹元整");
        assert_eq!(digit_uppercase(88.79).unwrap(), "捌拾捌元柒角玖分");
        assert_eq!(
            digit_uppercase(391293.45).unwrap(),
            "叁拾玖万壹仟贰佰玖拾叁元肆角伍分"
        );
        assert_eq!(
            digit_uppercase(2918923.12).unwrap(),
            "贰佰玖拾壹万捌仟玖佰贰拾叁元壹角贰分"
        );
        assert_eq!(
            digit_uppercase(12754894306.23).unwrap(),
            "壹佰贰拾柒亿伍仟肆佰捌拾玖万肆仟叁佰零陆元贰角叁分"
        );
    }

    #[test]
    fn test_digit_uppercase_errors() {
        assert!(digit_uppercase(-1.0).is_err());
        assert!(digit_uppercase(-100.5).is_err());
        assert!(digit_uppercase(f64::NAN).is_err());
        assert!(digit_uppercase(f64::INFINITY).is_err());
        assert!(digit_uppercase(100_000_000_000.0).is_err());
    }
}
