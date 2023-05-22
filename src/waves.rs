use biquad::*;
use gloo::console::console;
use crate::log;

pub trait Wave {
    fn at(&self, pos: f32) -> f32;
}

impl<F: Fn(f32) -> f32> Wave for F {
    fn at(&self, x: f32) -> f32 {
        self(x)
    }
}

pub fn sin(pos: f32) -> f32 {
    return (2.0 * std::f32::consts::PI * pos).sin();
}

pub fn square(pos: f32) -> f32 {
    return (2.0 * std::f32::consts::PI * pos).sin().signum();
}

pub fn triangle(pos: f32) -> f32 {
    return 2.0 * (2.0 * ((pos + 0.25) - (pos + 0.75).floor()).abs()) - 1.0;
}

pub fn sawtooth(pos: f32) -> f32 {
    return 2.0 * ((pos).floor() - (pos - 0.5));
}

/// distorts a wave. t is the amount of distortion. from 0 to 1, it distorts, from 0 to -1 it inverts the distortion.
pub fn distort_wave(f: Box<dyn Wave>, t: f32) -> Box<dyn Wave> {
    let res = move |pos: f32| {
        let y = f.at(pos);
        let distorted = dist_value(y.abs(), t) * y.signum();
        return distorted;
    };

    Box::new(res)
}

fn dist_value(y: f32, t: f32) -> f32 {
    if t > 0.0 && y > 1.0 - t {
        return 1.0;
    }

    if t < 0.0 && y < -t {
        return 0.0;
    }

    if t == 1.0 {
        return 1.0;
    }

    if t == -1.0 {
        return 0.0;
    }

    let mut z;

    if t > 0.0 {
        z = y / (1.0 - t);
    } else {
        z = (y + t) / (t + 1.0);
    }

    let mut d = 1.0 - t.abs();
    if t < 0.0 {
        d = 1.0 / d;
    }

    return z.powf(d);
}

pub fn skew_wave(f: Box<dyn Wave>, t: f32) -> Box<dyn Wave> {
    let skew = 0.5 - t * 0.5;

    let res = move |pos: f32| {
        let x = pos * 2.0 - 1.0;
        let skewed = (x.abs().powf(skew) * x.signum() + 1.0) * 0.5;

        return f.at(skewed);
    };

    Box::new(res)
}

pub fn sin_shape_wave(f: Box<dyn Wave>, t: f32) -> Box<dyn Wave> {
    let d = 0.5 - t * 0.5;

    let res = move |pos: f32| {
        let pos_sin = ((pos * 2.0 - 1.0) * std::f32::consts::PI * (0.5 + d)).sin() * 0.5 + 0.5;
        let x = (1.0 - d) * pos + d * pos_sin;
        return f.at(x);
    };

    Box::new(res)
}

pub fn sawtri(pos: f32, t: f32) -> f32 {
    let d = 1.0 - t;

    if d == 0.0 {
        return sawtooth(pos);
    }

    if pos * 4.0 < d {
        return pos * 4.0 / d;
    }

    if pos < 1.0 - d / 4.0 {
        let a = 2.0 / (d / 2.0 - 1.0);
        let b = 1.0 / (1.0 - d / 2.0);
        return a * pos + b;
    }

    return 4.0 * (pos - 1.0) / d;
}

pub fn transition_saw_to_tri(t: f32) -> Box<dyn Wave> {
    let res = move |pos: f32| {
        return sawtri(pos, t);
    };
    Box::new(res)
}

pub fn square_pulse(pos: f32, t: f32) -> f32 {
    const PULSE_WIDTH: f32 = 0.9;

    let d = 0.5 - t * PULSE_WIDTH * 0.5;

    if pos < d {
        return 1.0;
    }

    return -1.0;
}

pub fn transition_square_to_pulse(t: f32) -> Box<dyn Wave> {
    let res = move |pos: f32| {
        return square_pulse(pos, t);
    };
    Box::new(res)
}

pub fn lerp_func(a: Box<dyn Wave>, b: Box<dyn Wave>, t: f32) -> Box<dyn Wave> {
    let res = move |pos: f32| {
        return a.at(pos) * (1.0 - t) + b.at(pos) * t;
    };
    Box::new(res)
}

pub fn wave_table_from_func(wave: Box<dyn Wave>, wave_table_size: usize) -> Vec<f32> {
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    for n in 0..wave_table_size {
        let pos = (n + 1) as f32 / (wave_table_size + 1) as f32;
        wave_table.push(wave.at(pos));
    }

    return wave_table;
}

pub fn filter_table(waveform: Vec<f32>, amount: f32) -> Vec<f32> {
// Define the beta parameter
    let beta = 0.0 + (amount * 0.5 + 0.5) * 10.0;

// Calculate the Kaiser window
    let mut window = vec![0.0; waveform.len()];
    for i in 0..window.len() {
        let x = 2.0 * (i as f32) / ((window.len() - 1) as f32) - 1.0;
        window[i] = (1.0 - x.powi(2)).sqrt().powf(beta);
    }

// Apply the Kaiser window
    let mut windowed_waveform = vec![0.0; waveform.len()];
    for i in 0..waveform.len() {
        windowed_waveform[i] = waveform[i] * window[i];
    }

    return windowed_waveform;
}

pub fn normalize_table(table: Vec<f32>) -> Vec<f32> {
    let mut min = 0.0;
    let mut max = 0.0;

    for n in 0..table.len() {
        let val = table[n];
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }

    if min == max {
        return table;
    }

    let mut normalized_table: Vec<f32> = Vec::with_capacity(table.len());

    for n in 0..table.len() {
        let val = table[n];
        normalized_table.push(2.0 * (val - min) / (max - min) - 1.0);
    }

    return normalized_table;
}

pub fn sytrus_shape(shape: f32) -> Box<dyn Wave> {
    if shape < -0.5 {
        let alpha = shape * 2.0 + 2.0;
        return lerp_func(Box::new(sin), Box::new(triangle), alpha);
    } else if shape < 0.0 {
        let alpha = shape * 2.0 + 1.0;
        return transition_saw_to_tri(alpha);
    } else if shape < 0.5 {
        let alpha = shape * 2.0;
        return lerp_func(Box::new(sawtooth), Box::new(square), alpha);
    }

    let alpha = shape * 2.0 - 1.0;
    return transition_square_to_pulse(alpha);
}

pub fn wave_table_from_sytrus_params(shape: f32, tension: f32, skew: f32, sine_shaper: f32, pre_filter: f32, wave_table_size: usize) -> Vec<f32> {
    let shape = sytrus_shape(shape);

    let distorted = distort_wave(shape, tension);
    let sine_shaped = sin_shape_wave(distorted, sine_shaper);
    let skewed = skew_wave(sine_shaped, skew);

    let table = wave_table_from_func(skewed, wave_table_size);
    let filtered = filter_table(table, pre_filter);

    return normalize_table(filtered);
}
