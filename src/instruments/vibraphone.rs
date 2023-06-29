pub mod vibraphone {

use crate::sound_source::sound_source::{DynSoundSource, Instrument} ;
use crate::pure_tone::pure_tone::PureTone;
use crate::tremolo::tremolo::Tremolo;
use crate::ding_envelope::ding_envelope::DingEnvelope;


pub struct Vibraphone {}

impl Instrument for Vibraphone {
    fn play(&self, freq: f32, duration: f32, strength: f32) -> DynSoundSource {
        Box::new(
        Tremolo::new(5.0, 0.5, Box::new(
            DingEnvelope::new(2.0, duration, Box::new(
                PureTone::new(freq, strength, duration * 2.0)
            ))
        )))
    }
}

}