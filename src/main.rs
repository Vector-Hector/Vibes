mod midi;
mod synth;
mod bridge;
use std::sync::{Arc, Mutex};
use std::thread;
use rodio::{OutputStream, Sink, Source};
use crate::bridge::{BridgeSourceWrapper, MidiSynthBridge};
use crate::midi::listen;

fn main() {
    let synth = synth::get_example_wave_table_synth();
    let mut synth_bridge = MidiSynthBridge::new(Box::new(synth));
    synth_bridge.set_volume(1.0);
    let bridge = Arc::new(Mutex::new(synth_bridge));

    let mut bridge_for_midi = BridgeSourceWrapper::new(Arc::clone(&bridge));
    let bridge_for_audio = BridgeSourceWrapper::new(Arc::clone(&bridge));

    thread::spawn(move || {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        sink.append(bridge_for_audio);
        sink.sleep_until_end();
    });

    match listen(move |pressed, key, velocity| {bridge_for_midi.on_midi(pressed, key, velocity);} ) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}
