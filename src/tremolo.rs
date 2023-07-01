pub mod tremolo {

use crate::traits::traits::{SoundSource, DynSoundSource};

pub struct Tremolo
{
    freq: f32,
    depth: f32,
    source: DynSoundSource,
}

impl Tremolo {
    pub fn new(
        freq: f32,
        depth: f32,
        source: DynSoundSource,
    ) -> Self {
        if depth > 1.0 || depth < 0.0 {
            panic!("depth must be between 0.0 and 1.0");
        }
        if freq <= 0.0 {
            panic!("freq must be greater than 0.0");
        }
        Tremolo{
            freq: freq,
            depth: depth,
            source: source,
        }
    }
}

impl SoundSource for Tremolo {
    fn next_value(&self, t: f32) -> (f32, f32) {
        let source_val = (*self.source).next_value(t);
        let tremolo_gain = 1.0 - ((t * self.freq * 2.0 * std::f32::consts::PI).sin() + 1.0) * 0.5 * self.depth;
        (source_val.0 * tremolo_gain, source_val.1 * tremolo_gain)
    }

    fn duration(&self) -> f32 {
        (*self.source).duration()
    }
}

}