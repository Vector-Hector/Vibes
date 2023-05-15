pub trait Wave {
    fn at(&self, pos: f32) -> f32;
}

impl<F: Fn(f32) -> f32> Wave for F {
    fn at(&self, x: f32) -> f32 {
        self(x)
    }
}

pub fn sin_wave(pos: f32) -> f32 {
    return (2.0 * std::f32::consts::PI * pos).sin();
}

pub fn square_wave(pos: f32) -> f32 {
    return (2.0 * std::f32::consts::PI * pos).sin().signum();
}

pub fn triangle_wave(pos: f32) -> f32 {
    return 2.0 * (pos - (pos + 0.5).floor()).abs();
}

pub fn sawtooth_wave(pos: f32) -> f32 {
    return 2.0 * (pos - (pos + 0.5).floor());
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
        let pos = n as f32 / wave_table_size as f32;
        wave_table.push(wave.at(pos));
    }

    return wave_table;
}
