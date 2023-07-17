pub mod lfo {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource};

use crate::knob::knob::Knob;

pub struct LFO {
    dc_offset: f32,
    freq: f32,
    phase: f32,
    depth: Knob,
    duration: i32
}

impl LFO {
    pub fn new(dc_offset: f32, freq: f32, phase: f32, depth: Knob, duration: i32) -> Self {
        LFO { dc_offset: dc_offset, freq: freq, phase: phase, depth: depth, duration: duration }
    }
}
impl SoundSource for LFO {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        if n > self.duration {
            (0.0, 0.0)
        } else {
            let val = self.dc_offset
                + ((n as f32 * self.freq + self.phase) * 2.0 * std::f32::consts::PI).sin()
                    * self.depth.next_value(n);
            (val, val)
        }
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        use crate::dc::dc::DC;
        todo!();
        Box::new(Self::new(0.0, 0.0, 0.0, Knob::dc(0.0), 0))
    }
}

}