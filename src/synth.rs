use std::array::IntoIter;
use std::cmp::Ordering;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

use rodio::{Decoder, OutputStream, Sample, source::Source};
use rodio::cpal::FromSample;
use rodio::source::{Amplify, BltFilter, Buffered, Crossfade, Delay, FadeIn, Mix, Pausable, PeriodicAccess, Repeat, SamplesConverter, SkipDuration, Skippable, Speed, Stoppable, TakeDuration};
use crate::waves::{sin_wave, wave_table_from_func};

use super::bridge::{Message, Synth};

pub struct WaveTableSynth {
    /// the sample rate in hz
    sample_rate: u32,

    /// the wave table
    wave_table: Vec<f32>,

    /// the envelope (attack, decay, sustain, release)
    envelope: Envelope,

    /// the current sum of all the waves
    current_value: f32,
}

impl WaveTableSynth {
    pub(crate) fn new(sample_rate: u32, wave_table: Vec<f32>, envelope: Envelope) -> WaveTableSynth {
        return WaveTableSynth {
            sample_rate,
            wave_table,
            envelope,
            current_value: 0.0,
        };
    }

    /// Linearly interpolates between the two closest samples in the wave table.
    /// param frequency: The frequency of the note being played.
    /// param time: The time in seconds since the note was pressed.
    fn lerp(&self, frequency: f32, time: f32) -> f32 {
        let l = self.wave_table.len();
        let index = time * frequency * l as f32;
        let index_floor = index.floor() as usize % l;
        let index_ceil = index.ceil() as usize % l;
        let index_fraction = index.fract();

        let sample_floor = self.wave_table[index_floor];
        let sample_ceil = self.wave_table[index_ceil];

        return sample_floor + (sample_ceil - sample_floor) * index_fraction;
    }

    /// Updates the time since pressed and time since released fields of the message.
    fn next_message(&mut self, message: Message, volume: f32) -> Message {
        let mut tsp = message.time_since_pressed;
        let mut tsr = message.time_since_released;
        let dt = 1.0 / self.sample_rate as f32;

        if !message.is_released {
            tsp += dt;
        } else {
            tsr += dt;
        }

        return Message {
            key: message.key,
            velocity: message.velocity,
            is_released: message.is_released,
            time_since_pressed: tsp,
            time_since_released: tsr,
            start_volume: message.start_volume,
            last_volume: volume,
        };
    }
}

impl Synth for WaveTableSynth {
    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn reset(&mut self) {
        self.current_value = 0.0;
    }

    fn evaluate_message(&mut self, message: Message) -> Option<Message> {
        let (volume, is_active) = self.envelope.evaluate(message);
        if !is_active {
            return None;
        }

        let freq = 440.0 * 2.0f32.powf((message.key as f32 - 69.0) / 12.0);

        let value = self.lerp(freq, message.time_since_pressed + message.time_since_released);

        self.current_value += value * volume;
        return Some(self.next_message(message, volume));
    }

    fn get_sample(&mut self) -> f32 {
        return self.current_value;
    }
}

pub struct Envelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
}

impl Envelope {
    pub(crate) fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Envelope {
        return Envelope {
            attack,
            decay,
            sustain,
            release,
        };
    }

    fn evaluate(&mut self, message: Message) -> (f32, bool) {
        let time = message.time_since_pressed;
        let release_time = message.time_since_released;
        let velocity = message.velocity as f32 / 127.0;

        let mut value = 0.0;
        if time < self.attack {
            let start_volume = message.start_volume / velocity;
            value = start_volume + (1.0 - start_volume) * time / self.attack;
        } else if time < self.attack + self.decay {
            value = 1.0 - (time - self.attack) / self.decay * (1.0 - self.sustain);
        } else {
            value = self.sustain;
        }

        if message.is_released {
            if release_time >= self.release {
                return (0.0, false); // release is over
            }
            value *= 1.0 - (release_time / self.release);
        }

        return (value * velocity, true);
    }
}


pub fn get_example_wave_table_synth() -> WaveTableSynth {
    let wave_table = wave_table_from_func(Box::new(sin_wave), 64);

    let sample_rate = 44100;

    let envelope = Envelope::new(0.3, 0.4, 0.5, 0.5);

    return WaveTableSynth::new(sample_rate, wave_table, envelope);
}
