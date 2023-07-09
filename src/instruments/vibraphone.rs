pub mod vibraphone {

use crate::traits::traits::{DynSoundSource, Instrument} ;
use crate::knob::knob::Knob;
use crate::pure_tone::pure_tone::PureTone;
use crate::tremolo::tremolo::Tremolo;
use crate::ding_envelope::ding_envelope::DingEnvelope;


pub struct Vibraphone {
    sample_rate: i32,
}

impl Vibraphone {
    pub fn new(sample_rate: i32) -> Self {
        Vibraphone { sample_rate: sample_rate }
    }
}

impl Instrument for Vibraphone {
    fn play(&self, freq: f32, duration: i32, strength: f32) -> DynSoundSource {
        Box::new(
        Tremolo::new(5.0 / self.sample_rate as f32, 0.5, Box::new(
            DingEnvelope::new(2 * self.sample_rate, duration, Box::new(
                PureTone::new(Knob::dc(freq), Knob::dc(strength), duration * 2)
            ))
        )))
    }
}

}