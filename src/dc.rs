pub mod dc {

use num::Complex;

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData, ComplexSoundSource};

#[derive(Clone)]
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
    fn init_state(&self) -> SoundData {
        Box::new(0)
    }
    fn next_value(&self, n: i32, _state: &mut SoundData) -> (f32, f32) {
        if n > self.duration {
            (0.0, 0.0)
        } else {
            (self.value, self.value)
        }
    }
    fn duration(&self) -> i32 {
        self.duration
    }
    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let value = params[0].parse::<f32>().unwrap();
        let duration = params[1].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(Self::new(value, duration.round() as i32))
    }

}

#[derive(Clone)]
pub struct ComplexDC {
    value: Complex<f32>,
    duration: i32
}

impl ComplexDC {
    pub fn new(value: Complex<f32>, duration: i32) -> Self {
        ComplexDC { value: value, duration: duration }
    }
}
impl ComplexSoundSource for ComplexDC {
    fn init_state(&self) -> SoundData {
        Box::new(0)
    }
    fn next_value(&self, n: i32, _state: &mut SoundData) -> (Complex<f32>, Complex<f32>) {
        if n > self.duration {
            (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0))
        } else {
            (self.value, self.value)
        }
    }
    fn duration(&self) -> i32 {
        self.duration
    }
}

}