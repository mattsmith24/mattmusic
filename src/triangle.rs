pub mod triangle {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};
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
            true,
            duration
        ) }
    }
}

pub struct TriangleState {
    gen_state: SoundData
}

impl SoundSource for Triangle {
    fn init_state(&self) -> SoundData {
        Box::new(TriangleState { gen_state: self.generative_waveform.init_state() })
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<TriangleState>().unwrap();
        self.generative_waveform.next_value(n, &mut data.gen_state)
    }

    fn duration(&self) -> i32 {
        self.generative_waveform.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let freq = reader.get_knob(&params[0], 1.0 / reader.sample_rate as f32);
        let strength = reader.get_knob(&params[1], 1.0);
        let duration = params[2].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(Self::new(freq, strength, duration.round() as i32))
    }
}


}