pub mod ding_envelope {

use crate::sound_source::sound_source::{SoundSource, DynSoundSource};

pub struct DingEnvelope {
    decay: f32,
    duration: f32,
    source: DynSoundSource,
}

impl DingEnvelope {
    pub fn new(
        decay: f32,
        duration: f32,
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
    fn next_value(&self, t: f32) -> (f32, f32) {
        let source_val = (*self.source).next_value(t);
        let mut gain;
        const IMPULSE: f32 = 0.05;
        const FALLOFF: f32 = 0.1;
        if t < IMPULSE {
            gain = 1.0;
        } else if t < FALLOFF {
            // IMPULSE < t < FALLOFF, gain from 1.0 to 0.5
            gain = 1.0 + (t - IMPULSE) * -0.5 / (FALLOFF - IMPULSE);
        } else {
            // FALLOFF < t < decay, gain from 0.5 to 0
            gain = 0.5 + (t - FALLOFF) * -0.5 / (self.decay - FALLOFF);
        }
        const LIFT: f32 = 0.001;
        if t > self.duration - LIFT {
            // ramp down to end of note to avoid discontinuity
            gain *= 1.0 - (t - (self.duration - LIFT)) / LIFT;
        }
        if gain < 0.0 || t > self.duration {
            gain = 0.0;
        }
        (source_val.0 * gain, source_val.1 * gain)
    }

    fn duration(&self) -> f32 {
        self.decay.min(self.duration).min((*self.source).duration())
    }
}

}
