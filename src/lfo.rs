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
        let dc_offset = params[0].parse::<f32>().unwrap();
        let freq = params[1].parse::<f32>().unwrap() / reader.sample_rate as f32;
        let phase = params[2].parse::<f32>().unwrap() / reader.sample_rate as f32;
        let depth = reader.get_knob(&params[3], 1.0);
        let duration = params[4].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(Self::new(dc_offset, freq, phase, depth, duration.round() as i32))
    }
}

}