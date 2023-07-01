pub mod saw {

use crate::traits::traits::SoundSource;
use crate::knob::knob::Knob;
use crate::generative_waveform::generative_waveform::GenerativeWaveform;

pub struct Saw {
    generative_waveform: GenerativeWaveform
}

impl Saw {
    pub fn new(
        sample_rate: f32,
        freq: Knob,
        gain: Knob,
        duration: f32
    ) -> Self {
        Saw { generative_waveform: GenerativeWaveform::new(
            sample_rate,
            freq,
            1,
            1.0,
            gain,
            duration
        ) }
    }
}

impl SoundSource for Saw {
    fn next_value(&self, t: f32) -> (f32, f32) {
        self.generative_waveform.next_value(t)
    }

    fn duration(&self) -> f32 {
        self.generative_waveform.duration()
    }
}


}