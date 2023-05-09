use wasm_bindgen::prelude::*;

use web_sys::{MessageEvent, console};
use js_sys::Float32Array;
use rand::Rng;
use serde_derive::{Serialize, Deserialize};

// event data: { type: 'calculateSamples', length: 128 }

#[derive(Serialize, Deserialize)]
pub struct CalculateSamplesMessage {
    pub length: u32,

    #[serde(rename = "type")]
    pub typ: String,
}

#[wasm_bindgen]
pub fn calculate_samples(event: MessageEvent) -> Float32Array {
    let msg: CalculateSamplesMessage = serde_wasm_bindgen::from_value(event.data()).unwrap();
    let len = msg.length;

    let samples = Float32Array::new_with_length(len);
    let mut rng = rand::thread_rng();

    // populate samples with random values from -1 to 1
    for i in 0..len {
        let val = rng.gen::<f32>() * 2.0 - 1.0;
        samples.set_index(i, val);
    }

    return samples;
}
