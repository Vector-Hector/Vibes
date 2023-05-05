use std::collections::HashMap;
use std::time::Duration;
use rodio::Source;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
pub(crate) struct Message {
    pub(crate) key: u8,
    pub(crate) velocity: u8,
    pub(crate) last_volume: f32,
    pub(crate) start_volume: f32,
    pub(crate) is_released: bool,
    pub(crate) time_since_pressed: f32,
    pub(crate) time_since_released: f32,
}

pub(crate) struct MidiSynthBridge {
    sample_rate: u32,
    messages: HashMap<u8, Message>,
    synth: Box<dyn Synth>,
    volume: f32,
}

impl MidiSynthBridge {
    pub(crate) fn new(synth: Box<dyn Synth>) -> MidiSynthBridge {
        let sr = synth.sample_rate();

        return MidiSynthBridge {
            messages: HashMap::new(),
            synth,
            sample_rate: sr,
            volume: 1.0,
        };
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    fn on_midi(&mut self, pressed: bool, key: u8, velocity: u8) {
        let message = self.messages.entry(key).or_insert(Message {
            key,
            velocity,
            last_volume: 0.0,
            start_volume: 0.0,
            is_released: false,
            time_since_pressed: 0.0,
            time_since_released: 0.0,
        });

        message.velocity = velocity;
        message.is_released = !pressed;
        if pressed {
            message.start_volume = message.last_volume; // ensure smooth transition from last note
            message.time_since_pressed = 0.0;
        } else {
            message.time_since_released = 0.0;
        }
    }

    fn get_sample(&mut self) -> f32 {
        self.synth.reset();

        let mut to_remove = Vec::new();

        for (_, message) in self.messages.iter_mut() {
            let evaluated_message = self.synth.evaluate_message(message.clone());
            if let Some(evaluated_message) = evaluated_message {
                *message = evaluated_message;
                continue;
            }
            to_remove.push(message.key);
        }

        for key in to_remove {
            self.messages.remove(&key);
        }

        return self.synth.get_sample() * self.volume;
    }
}

pub(crate) trait Synth {
    fn sample_rate(&self) -> u32;
    fn reset(&mut self);
    fn evaluate_message(&mut self, message: Message) -> Option<Message>;
    fn get_sample(&mut self) -> f32;
}

pub(crate) struct BridgeSourceWrapper {
    bridge: Arc<Mutex<MidiSynthBridge>>,
}

impl BridgeSourceWrapper {
    pub(crate) fn new(bridge: Arc<Mutex<MidiSynthBridge>>) -> BridgeSourceWrapper {
        return BridgeSourceWrapper { bridge };
    }

    pub fn on_midi(&mut self, pressed: bool, key: u8, velocity: u8) {
        let mut bridge = self.get_bridge();
        bridge.on_midi(pressed, key, velocity);
    }

    fn get_bridge(&self) -> std::sync::MutexGuard<MidiSynthBridge> {
        return self.bridge.lock().unwrap();
    }
}

impl Iterator for BridgeSourceWrapper {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut bridge = self.get_bridge();
        return Some(bridge.get_sample());
    }
}

 impl Source for BridgeSourceWrapper {
    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        let mut bridge = self.get_bridge();
        return bridge.sample_rate;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

unsafe impl Send for BridgeSourceWrapper {


}

