#[macro_export]
macro_rules! log {
    () => {
use web_sys::console;
use wasm_bindgen::JsValue;
        console::log_1(&JsValue::from_str("\n"));
    };
    ($($arg:tt)*) => {{
use web_sys::console;
use wasm_bindgen::JsValue;
        let str = format!($($arg)*);
        let js_val = JsValue::from_str(&str);
        console::log_1(&js_val);
    }};
}
