pub mod dc {

use crate::traits::traits::SoundSource;

pub struct DC {
    value: f32,
    duration: i32
}

impl DC {
    pub fn new(value: f32, duration: i32) -> Self {
        DC { value: value, duration: duration }
    }
}
impl SoundSource for DC {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        if n > self.duration {
            (0.0, 0.0)
        } else {
            (self.value, self.value)
        }
    }
    fn duration(&self) -> i32 {
        self.duration
    }
}

}