use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioContext, AudioNode, AudioWorkletNode, console, GainNode, Request, Response, Window};

use crate::log;

pub struct Master {
    audio_context: AudioContext,
    master_processor: AudioWorkletNode,
}

impl Master {
    pub async fn new() -> Result<Master, JsValue> {
        let audio_context = AudioContext::new()?;
        JsFuture::from(audio_context
            .audio_worklet()?
            .add_module("static/worker/bundled_rust_audio_processor.js")?
        ).await?;

        let gain_node = GainNode::new(&audio_context)?;
        gain_node.gain().set_value(0.1);

        let gain_node_as_audio_node: &AudioNode = gain_node.as_ref();
        gain_node_as_audio_node.connect_with_audio_node(&audio_context.destination())?;

        let master_processor = AudioWorkletNode::new(&audio_context, "master-processor")?;
        master_processor.connect_with_audio_node(gain_node_as_audio_node)?;

        let master_processor_port = master_processor.port()?;
        let wasm_module = fetch_and_compile_wasm("static/worker/audio_worker_bg.wasm").await?;
        let wasm_module_message = create_message("wasmModule", wasm_module);
        master_processor_port.post_message(&wasm_module_message)?;

        Ok(Master {
            audio_context,
            master_processor,
        })
    }

    pub fn post_message(
        &self,
        msg: &JsValue,
    ) -> Result<(), JsValue> {
        self.master_processor
            .port()?
            .post_message(msg)?;

        Ok(())
    }

    pub async fn play(&self) -> Result<(), JsValue> {
        log!("Master play...");
        JsFuture::from(self.audio_context.resume()?).await?;
        Ok(())
    }

    pub fn set_wave_table(&self, wave_table: Vec<f32>) {
        let wave_table_message = create_message("waveTable", serde_wasm_bindgen::to_value(&wave_table).unwrap());
        log!("Master set_wave_table...");
        self.post_message(&wave_table_message).unwrap();
    }
}

unsafe impl Send for Master {
}

async fn fetch_and_compile_wasm(wasm_url: &str) -> Result<JsValue, JsValue> {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let request = Request::new_with_str(wasm_url).unwrap();

    let response_js = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response = Response::from(response_js);

    let wasm_buffer_js = JsFuture::from(response.array_buffer()?).await?;
    let wasm_buffer = js_sys::Uint8Array::new(&wasm_buffer_js);

    let wasm_module = JsFuture::from(js_sys::WebAssembly::compile(&wasm_buffer)).await?;

    Ok(wasm_module)
}

fn create_message(typ: &str, value: JsValue) -> JsValue {
    let message = js_sys::Object::new();
    js_sys::Reflect::set(&message, &JsValue::from_str("type"), &JsValue::from_str(typ)).unwrap();
    js_sys::Reflect::set(&message, &JsValue::from_str("value"), &value).unwrap();
    message.into()
}
