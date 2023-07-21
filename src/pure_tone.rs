pub mod pure_tone {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource};

use crate::knob::knob::Knob;

pub struct PureTone {
    freq: Knob,
    gain: Knob,
    duration: i32,
}

impl PureTone {
    pub fn new(
        freq: Knob,
        gain: Knob,
        duration: i32
    ) -> Self {
        PureTone{
            freq: freq,
            gain: gain,
            duration: duration,
        }
    }
}

impl SoundSource for PureTone {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        if n > self.duration {
            (0.0, 0.0)
        } else {
            let val = (n as f32 * self.freq.next_value(n) * 2.0 * std::f32::consts::PI).sin()
                * self.gain.next_value(n);
            (val, val)
        }
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(_params: &Vec::<String>, _reader: &mut SongReader) -> DynSoundSource {
        Box::new(Self::new(Knob::dc(0.0), Knob::dc(0.0), 0))
    }
}
}