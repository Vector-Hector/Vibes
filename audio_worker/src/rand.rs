pub struct SimpleRng {
    seed: u32,

}

impl SimpleRng {
    pub fn new(seed: u32) -> Self {
        SimpleRng { seed }
    }

    pub fn next(&mut self) -> f32 {
        self.seed = xorshift(self.seed);
        (self.seed as f32) / (u32::MAX as f32)
    }
}

fn xorshift(mut x: u32) -> u32 {
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}
