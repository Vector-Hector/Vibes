use std::any::{Any, type_name};
use std::borrow::{Borrow, BorrowMut};
use std::mem::discriminant;
use std::ops::{Deref, DerefMut};
use js_sys::Float32Array;
use wasm_bindgen::prelude::*;
use crate::bridge::{MidiSynthBridge, Synth};
use crate::synth::{Envelope, WaveTableSynth};
use crate::waves::{lerp_func, sawtooth_wave, sin_wave, square_wave, triangle_wave, wave_table_from_func};

mod bridge;
mod synth;
mod waves;
mod log;
mod rand;

static mut SYNTH: Option<MidiSynthBridge> = None;

fn create_synth() -> MidiSynthBridge {

    // create the wave function
    let sin = Box::new(sin_wave);
    let _square = Box::new(square_wave);
    let _saw = Box::new(sawtooth_wave);
    let _tri = Box::new(triangle_wave);

    let wave = sin; // lerp_func(sin, sin, 0.5);

    // create the synth
    let wave_table = wave_table_from_func(wave, 64);
    let sample_rate = 44100;
    let envelope = Envelope::new(0.3, 0.4, 0.8, 0.5);
    let synth = WaveTableSynth::new(sample_rate, wave_table, envelope);

    // create the bridge
    let mut synth_bridge = MidiSynthBridge::new(Box::new(synth));
    synth_bridge.set_volume(1.0);

    return synth_bridge;
}

fn get_synth() -> &'static mut MidiSynthBridge {
    unsafe {
        match SYNTH {
            Some(ref mut synth) => synth,
            None => {
                SYNTH = Some(create_synth());
                get_synth()
            }
        }
    }
}

#[wasm_bindgen]
pub fn calculate_samples(len: u32) -> Float32Array {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let samples = Float32Array::new_with_length(len);

    let synth = get_synth();

    // populate samples with random values from -1 to 1
    for i in 0..len {
        let val = synth.get_sample();
        samples.set_index(i, val);
    }

    return samples;
}

#[wasm_bindgen]
pub fn on_midi(is_active: bool, key: u8, velocity: u8) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let synth = get_synth();

    synth.on_midi(is_active, key, velocity);
}

trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[wasm_bindgen]
pub fn set_wave_table(wave_table: Float32Array) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let synth = get_synth().get_synth();
    synth.set_wave_table(Float32Array::to_vec(&wave_table));
}
