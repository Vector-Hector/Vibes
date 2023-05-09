
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker};

#[wasm_bindgen(module="/src/processor-bridge.js")]
extern "C" {
    #[wasm_bindgen(js_name = "setWorker")]
    pub fn set_worker(worker: &Worker);

    #[wasm_bindgen(js_name = "setOnMessage")]
    pub fn set_on_message(callback: &Closure<dyn FnMut(MessageEvent)>);

    #[wasm_bindgen(js_name = "sendSamples")]
    pub fn send_samples(samples: &[f32], buffer: u32);

    pub fn initialize();

    pub fn play();
}
