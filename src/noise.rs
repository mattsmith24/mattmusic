pub mod noise {

use rand::Rng;
use crate::traits::traits::SoundSource;

pub struct Noise {
    duration: i32,
}

impl Noise {
    pub fn new(
        duration: i32
    ) -> Self {
        Noise{
            duration: duration,
        }
    }
}

impl SoundSource for Noise {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        if n > self.duration {
            (0.0, 0.0)
        } else {
            let mut rng = rand::thread_rng();
            let val: f32 = rng.gen_range(0.0..1.0);
            (val, val)
        }
    }

    fn duration(&self) -> i32 {
        self.duration
    }
}
}