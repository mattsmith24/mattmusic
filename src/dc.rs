pub mod dc {

use crate::read_song::read_song::YAMLFormat;
use crate::traits::traits::{SoundSource, DynSoundSource};

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
    fn from_yaml(params: &Vec::<String>, yaml: &YAMLFormat, sample_rate: i32) -> DynSoundSource {
        let value = params[0].parse::<f32>().unwrap();
        let duration = params[1].parse::<f32>().unwrap() * sample_rate as f32;
        Box::new(Self::new(value, duration.round() as i32))
    }

}

}