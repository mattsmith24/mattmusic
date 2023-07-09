pub mod saw {

use crate::traits::traits::SoundSource;
use crate::knob::knob::Knob;
use crate::generative_waveform::generative_waveform::GenerativeWaveform;

pub struct Saw {
    generative_waveform: GenerativeWaveform
}

impl Saw {
    pub fn new(
        freq: Knob,
        gain: Knob,
        duration: i32
    ) -> Self {
        Saw { generative_waveform: GenerativeWaveform::new(
            freq,
            1,
            1,
            gain,
            duration
        ) }
    }
}

impl SoundSource for Saw {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        self.generative_waveform.next_value(n)
    }

    fn duration(&self) -> i32 {
        self.generative_waveform.duration()
    }
}


}