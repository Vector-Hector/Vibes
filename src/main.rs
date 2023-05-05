mod midi;
mod synth;
mod bridge;
mod waves;

use std::sync::{Arc, Mutex};
use std::thread;
use rodio::{OutputStream, Sink, Source};
use crate::bridge::{BridgeSourceWrapper, MidiSynthBridge};
use crate::midi::listen;
use crate::synth::{Envelope, WaveTableSynth};
use crate::waves::{lerp_func, sawtooth_wave, sin_wave, square_wave, wave_table_from_func};

fn main() {
    // create the wave function
    let sin = Box::new(sin_wave);
    let square = Box::new(square_wave);
    let saw = Box::new(sawtooth_wave);
    let tri = Box::new(waves::triangle_wave);

    let wave = lerp_func(saw, tri, 0.5);

    // create the synth
    let wave_table = wave_table_from_func(wave, 64);
    let sample_rate = 44100;
    let envelope = Envelope::new(0.3, 0.4, 0.5, 0.5);
    let synth = WaveTableSynth::new(sample_rate, wave_table, envelope);

    // create the bridge
    let mut synth_bridge = MidiSynthBridge::new(Box::new(synth));
    synth_bridge.set_volume(1.0);
    let bridge = Arc::new(Mutex::new(synth_bridge));

    // prepare for multi threading
    let mut bridge_for_midi = BridgeSourceWrapper::new(Arc::clone(&bridge));
    let bridge_for_audio = BridgeSourceWrapper::new(Arc::clone(&bridge));

    // start the audio thread
    thread::spawn(move || {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        sink.append(bridge_for_audio);
        sink.sleep_until_end();
    });

    // start the midi thread
    match listen(move |pressed, key, velocity| {bridge_for_midi.on_midi(pressed, key, velocity);} ) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}
