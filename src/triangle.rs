pub mod triangle {

use crate::traits::traits::SoundSource;
use crate::knob::knob::Knob;
use crate::generative_waveform::generative_waveform::GenerativeWaveform;

pub struct Triangle {
    generative_waveform: GenerativeWaveform
}

impl Triangle {
    pub fn new(
        freq: Knob,
        gain: Knob,
        duration: i32
    ) -> Self {
        Triangle { generative_waveform: GenerativeWaveform::new(
            freq,
            2,
            2,
            gain,
            duration
        ) }
    }
}

impl SoundSource for Triangle {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        self.generative_waveform.next_value(n)
    }

    fn duration(&self) -> i32 {
        self.generative_waveform.duration()
    }
}


}