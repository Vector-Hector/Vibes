use std::borrow::Borrow;
use gloo::console::console;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, GainNode, OscillatorNode, OscillatorType, Worker, console, MessageEvent};
use yew::prelude::*;
use serde::{Serialize, Deserialize};

use crate::processor_bridge;
use crate::processor_bridge::send_samples;

#[derive(Serialize, Deserialize)]
pub struct CalculateSamplesMessage {
    pub length: usize,

    #[serde(rename = "type")]
    pub typ: String,
}

impl CalculateSamplesMessage {
    pub fn new(length: usize) -> CalculateSamplesMessage {
        return CalculateSamplesMessage {
            length,
            typ: "calculateSamples".to_string(),
        };
    }
}

#[derive(Serialize, Deserialize)]
pub struct SamplesMessage {
    pub samples: Vec<f32>,

    pub buffer: u32,

    #[serde(rename = "type")]
    pub typ: String,

}

impl SamplesMessage {
    pub fn new(samples: Vec<f32>) -> SamplesMessage {
        return SamplesMessage {
            samples,
            buffer: 0,
            typ: "samples".to_string(),
        };
    }
}

fn init() -> Worker {
    processor_bridge::initialize();

    let wrk = Worker::new("/static/bundled_worker.js").unwrap();

    let worker_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
        let msg: SamplesMessage = serde_wasm_bindgen::from_value(event.data()).unwrap();

        send_samples(msg.samples.as_slice(), msg.buffer)
    }) as Box<dyn FnMut(MessageEvent)>);

    wrk.add_event_listener_with_callback("message", worker_callback.as_ref().unchecked_ref()).unwrap();

    processor_bridge::set_worker(&wrk);

    worker_callback.forget();

    return wrk;
}

fn play(worker: &Worker) -> Result<(), JsValue> {
    let csm = CalculateSamplesMessage::new(2048);

    worker.post_message(&serde_wasm_bindgen::to_value(csm.borrow())?);
    worker.post_message(&serde_wasm_bindgen::to_value(csm.borrow())?);

    processor_bridge::play();

    Ok(())
}

#[function_component(App)]
pub fn app() -> Html {
    let worker = use_state(init);

    return html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "lel" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        <button onclick={move |_| {
            play(&*worker);
        }}>{ "Play" }</button>
        </main>
    }
}
