pub mod pure_tone {

use crate::sound_source::sound_source::SoundSource;

pub struct PureTone {
    freq: f32,
    gain: f32,
    duration: f32,
}

impl PureTone {
    pub fn new(
        freq: f32,
        gain: f32,
        duration: f32
    ) -> Self {
        if freq <= 0.0 {
            panic!("freq must be greater than 0.0");
        }
        PureTone{
            freq: freq,
            gain: gain,
            duration: duration,
        }
    }
}

impl SoundSource for PureTone {
    fn next_value(&self, t: f32) -> (f32, f32) {
        if t > self.duration {
            (0.0, 0.0)
        } else {
            let val = (t * self.freq * 2.0 * std::f32::consts::PI).sin() * self.gain;
            (val, val)
        }
    }

    fn duration(&self) -> f32 {
        self.duration
    }
}
}