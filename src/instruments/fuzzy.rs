pub mod fuzzy {

use crate::sound_source::sound_source::{DynSoundSource, Instrument} ;
use crate::square::square::Square;
use crate::tremolo::tremolo::Tremolo;
use crate::ding_envelope::ding_envelope::DingEnvelope;


pub struct Fuzzy {
    sample_rate: f32,
}

impl Fuzzy {
    pub fn new(sample_rate: f32) -> Self {
        Fuzzy { sample_rate: sample_rate }
    }
}

impl Instrument for Fuzzy {
    fn play(&self, freq: f32, duration: f32, strength: f32) -> DynSoundSource {
        Box::new(
        Tremolo::new(5.0, 0.5, Box::new(
            DingEnvelope::new(2.0, duration, Box::new(
                Square::new(self.sample_rate, freq, strength, duration * 2.0)
            ))
        )))
    }
}

}