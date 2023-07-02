pub mod lfo {

use crate::traits::traits::SoundSource;
use crate::knob::knob::Knob;

pub struct LFO {
    dc_offset: f32,
    freq: f32,
    phase: f32,
    depth: Knob,
    duration: f32
}

impl LFO {
    pub fn new(dc_offset: f32, freq: f32, phase: f32, depth: Knob, duration: f32) -> Self {
        LFO { dc_offset: dc_offset, freq: freq, phase: phase, depth: depth, duration: duration }
    }
}
impl SoundSource for LFO {
    fn next_value(&self, t: f32) -> (f32, f32) {
        if t > self.duration {
            (0.0, 0.0)
        } else {
            let val = self.dc_offset
                + ((t * self.freq + self.phase) * 2.0 * std::f32::consts::PI).sin()
                    * self.depth.next_value(t);
            (val, val)
        }
    }

    fn duration(&self) -> f32 {
        self.duration
    }
}

}