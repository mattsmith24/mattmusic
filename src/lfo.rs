pub mod lfo {

use crate::traits::traits::{SoundSource, DynSoundSource};
use crate::knob::knob::Knob;
use crate::mix::mix::Mix;
use crate::pure_tone::pure_tone::PureTone;
use crate::dc::dc::DC;

pub struct LFO {
    duration: f32,
    source: Mix
}

impl LFO {
    pub fn new(dc_offset: f32, freq: f32, depth: f32, duration: f32,) -> Self {
        let mut mix = Mix::new();
        let dc = Box::new(DC::new(dc_offset, duration));
        let wave = Box::new(PureTone::new(Knob::dc(freq), Knob::dc(depth), duration));
        mix.add(dc);
        mix.add(wave);
        LFO { duration: duration, source: mix }
    }
}
impl SoundSource for LFO {
    fn next_value(&self, t: f32) -> (f32, f32) {
        if t > self.duration {
            (0.0, 0.0)
        } else {
            self.source.next_value(t)
        }
    }
    fn duration(&self) -> f32 {
        self.duration
    }
}

}