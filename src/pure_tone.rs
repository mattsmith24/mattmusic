pub mod pure_tone {

use crate::traits::traits::SoundSource;
use crate::knob::knob::Knob;

pub struct PureTone {
    freq: Knob,
    gain: Knob,
    duration: f32,
}

impl PureTone {
    pub fn new(
        freq: Knob,
        gain: Knob,
        duration: f32
    ) -> Self {
        PureTone{
            freq: freq,
            gain: gain,
            duration: duration,
        }
    }
}

impl SoundSource for PureTone {
    fn next_value(&mut self, t: f32) -> (f32, f32) {
        if t > self.duration {
            (0.0, 0.0)
        } else {
            let val = (t * self.freq.next_value(t) * 2.0 * std::f32::consts::PI).sin()
                * self.gain.next_value(t);
            (val, val)
        }
    }

    fn duration(&self) -> f32 {
        self.duration
    }
}
}