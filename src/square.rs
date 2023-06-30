pub mod square {

use crate::sound_source::sound_source::SoundSource;
use crate::generative_waveform::generative_waveform::GenerativeWaveform;

pub struct Square {
    generative_waveform: GenerativeWaveform
}

impl Square {
    pub fn new(
        sample_rate: f32,
        freq: f32,
        gain: f32,
        duration: f32
    ) -> Self {
        Square { generative_waveform: GenerativeWaveform::new(
            sample_rate,
            freq,
            2,
            1.0,
            gain,
            duration
        ) }
    }
}

impl SoundSource for Square {
    fn next_value(&self, t: f32) -> (f32, f32) {
        self.generative_waveform.next_value(t)
    }

    fn duration(&self) -> f32 {
        self.generative_waveform.duration()
    }
}


}