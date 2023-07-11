pub mod sine {

use crate::traits::traits::SoundSource;
use crate::knob::knob::Knob;
use crate::generative_waveform::generative_waveform::GenerativeWaveform;

pub struct Sine {
    generative_waveform: GenerativeWaveform
}

impl Sine {
    pub fn new(
        freq: Knob,
        gain: Knob,
        duration: i32
    ) -> Self {
        Sine { generative_waveform: GenerativeWaveform::new(
            freq,
            1000000000, // A really high number so we don't add any frequencies
            1,
            gain,
            duration
        ) }
    }
}

impl SoundSource for Sine {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        self.generative_waveform.next_value(n)
    }

    fn duration(&self) -> i32 {
        self.generative_waveform.duration()
    }
}


}