use bevy::prelude::*;

pub fn rng_plugin(app:&mut App){
    app.insert_resource(SimpleRng::new(111));
}

#[derive(Resource)]
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed.max(1) } // avoid 0 state for xorshift
    }

    // xorshift64*
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    /// Generates a random float in [0.0, 1.0)
    pub fn next_scaled(&mut self) -> f32 {
        let bits = 0x3F800000 | (self.next_u32() >> 9); // 1.xxxxxx... in IEEE 754
        f32::from_bits(bits) - 1.0
    }

}

