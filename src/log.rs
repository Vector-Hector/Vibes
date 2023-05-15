#[macro_export]
macro_rules! log {
    () => {
        console::log_1(&JsValue::from_str("\n"));
    };
    ($($arg:tt)*) => {{
        let str = format!($($arg)*);
        let js_val = JsValue::from_str(&str);
        console::log_1(&js_val);
    }};
}
