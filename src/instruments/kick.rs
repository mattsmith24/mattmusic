pub mod kick {

use crate::traits::traits::{DynSoundSource, Instrument} ;
use crate::square::square::Square;
use crate::pure_tone::pure_tone::PureTone;
use crate::mix::mix::Mix;

pub struct Kick {
    sample_rate: f32,
}

impl Kick {
    pub fn new(sample_rate: f32) -> Self {
        Kick { sample_rate: sample_rate }
    }
}

impl Instrument for Kick {
    fn play(&self, _freq: f32, duration: f32, strength: f32) -> DynSoundSource {
        let mut res = Box::new(Mix::new());
        (*res).add(Box::new(Square::new(self.sample_rate, 100.0, strength, 0.01)));
        (*res).add(Box::new(PureTone::new(50.0, 0.0, duration)));
        res
    }
}

}