pub mod saw {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

use crate::knob::knob::Knob;
use crate::generative_waveform::generative_waveform::GenerativeWaveform;

#[derive(Clone)]
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
            true,
            duration
        ) }
    }
}

struct SawData {
    gen_data: SoundData,
}

impl SoundSource for Saw {
    fn init_state(&self) -> SoundData {
        Box::new(SawData { gen_data: self.generative_waveform.init_state() })
    }
    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<SawData>().unwrap();
        self.generative_waveform.next_value(n, &mut data.gen_data)
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