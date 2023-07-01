pub mod dc {

use crate::traits::traits::SoundSource;

pub struct DC {
    value: f32,
    duration: f32
}

impl DC {
    pub fn new(value: f32, duration: f32) -> Self {
        DC { value: value, duration: duration }
    }
}
impl SoundSource for DC {
    fn next_value(&self, t: f32) -> (f32, f32) {
        if t > self.duration {
            (0.0, 0.0)
        } else {
            (self.value, self.value)
        }
    }
    fn duration(&self) -> f32 {
        self.duration
    }
}

}