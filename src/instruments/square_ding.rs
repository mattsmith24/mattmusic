pub mod square_ding {

use crate::traits::traits::{DynSoundSource, Instrument} ;
use crate::knob::knob::Knob;
use crate::square::square::Square;
use crate::tremolo::tremolo::Tremolo;
use crate::ding_envelope::ding_envelope::DingEnvelope;


pub struct SquareDing {
    sample_rate: f32,
}

impl SquareDing {
    pub fn new(sample_rate: f32) -> Self {
        SquareDing { sample_rate: sample_rate }
    }
}

impl Instrument for SquareDing {
    fn play(&self, freq: f32, duration: f32, strength: f32) -> DynSoundSource {
        Box::new(
        Tremolo::new(5.0, 0.5, Box::new(
            DingEnvelope::new(2.0, duration, Box::new(
                Square::new(self.sample_rate, Knob::dc(freq), Knob::dc(strength), duration * 2.0)
            ))
        )))
    }
}

}