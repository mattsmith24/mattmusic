pub mod ding_envelope {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource};

pub struct DingEnvelope {
    decay: i32,
    duration: i32,
    source: DynSoundSource,
}

impl DingEnvelope {
    pub fn new(
        decay: i32,
        duration: i32,
        source: DynSoundSource
    ) -> Self {
        DingEnvelope{
            decay: decay,
            duration: duration,
            source: source
        }
    }
}

impl SoundSource for DingEnvelope {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        let source_val = (*self.source).next_value(n);
        let mut gain;
        const IMPULSE: i32 = 220;
        const FALLOFF: i32 = 4400;
        if n < IMPULSE {
            gain = 1.0;
        } else if n < FALLOFF {
            // IMPULSE < t < FALLOFF, gain from 1.0 to 0.5
            gain = 1.0 + (n - IMPULSE) as f32 * -0.5 / (FALLOFF - IMPULSE) as f32;
        } else {
            // FALLOFF < t < decay, gain from 0.5 to 0
            gain = 0.5 + (n - FALLOFF) as f32 * -0.5 / (self.decay - FALLOFF) as f32;
        }
        const LIFT: i32 = 44;
        if n > self.duration - LIFT {
            // ramp down to end of note to avoid discontinuity
            gain *= 1.0 - (n - (self.duration - LIFT)) as f32 / LIFT as f32;
        }
        if gain < 0.0 || n > self.duration {
            gain = 0.0;
        }
        (source_val.0 * gain, source_val.1 * gain)
    }

    fn duration(&self) -> i32 {
        self.decay.min(self.duration).min((*self.source).duration())
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        use crate::dc::dc::DC;
        todo!();
        Box::new(Self::new(0, 0, Box::new(DC::new(0.0, 0))))
    }

}

}
