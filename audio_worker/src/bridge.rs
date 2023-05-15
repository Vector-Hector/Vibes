use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct KeyState {
    pub(crate) key: u8,
    pub(crate) velocity: u8,
    pub(crate) last_volume: f32,
    pub(crate) start_volume: f32,
    pub(crate) is_released: bool,
    pub(crate) time_since_pressed: f32,
    pub(crate) time_since_released: f32,
}


pub struct MidiSynthBridge {
    messages: HashMap<u8, KeyState>,
    synth: Box<dyn Synth>,
    volume: f32,
}

impl MidiSynthBridge {
    pub fn new(synth: Box<dyn Synth>) -> MidiSynthBridge {
        return MidiSynthBridge {
            messages: HashMap::new(),
            synth,
            volume: 1.0,
        };
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    pub fn on_midi(&mut self, pressed: bool, key: u8, velocity: u8) {
        let message = self.messages.entry(key).or_insert(KeyState {
            key,
            velocity,
            last_volume: 0.0,
            start_volume: 0.0,
            is_released: false,
            time_since_pressed: 0.0,
            time_since_released: 0.0,
        });

        message.is_released = !pressed;
        if pressed {
            message.start_volume = message.last_volume; // ensure smooth transition from last note
            message.time_since_pressed = 0.0;
            message.velocity = velocity;
        } else {
            message.time_since_released = 0.0;
            message.start_volume = message.last_volume; // ensure smooth transition when releasing quickly
        }
    }

    pub fn get_sample(&mut self) -> f32 {
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

pub trait Synth {
    fn sample_rate(&self) -> u32;
    fn reset(&mut self);
    fn evaluate_message(&mut self, message: KeyState) -> Option<KeyState>;
    fn get_sample(&mut self) -> f32;
}

