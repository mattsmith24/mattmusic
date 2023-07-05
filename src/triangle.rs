pub mod triangle {

use crate::traits::traits::SoundSource;
use crate::knob::knob::Knob;
use crate::generative_waveform::generative_waveform::GenerativeWaveform;

pub struct Triangle {
    generative_waveform: GenerativeWaveform
}

impl Triangle {
    pub fn new(
        sample_rate: f32,
        freq: Knob,
        gain: Knob,
        duration: f32
    ) -> Self {
        Triangle { generative_waveform: GenerativeWaveform::new(
            sample_rate,
            freq,
            2,
            2.0,
            gain,
            duration
        ) }
    }
}

impl SoundSource for Triangle {
    fn next_value(&mut self, t: f32) -> (f32, f32) {
        self.generative_waveform.next_value(t)
    }

    fn duration(&self) -> f32 {
        self.generative_waveform.duration()
    }
}


}