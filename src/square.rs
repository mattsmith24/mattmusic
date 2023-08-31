pub mod square {

use std::sync::{Arc, Mutex};

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

use crate::knob::knob::Knob;
use crate::generative_waveform::generative_waveform::GenerativeWaveform;

pub struct Square {
    generative_waveform: GenerativeWaveform
}

impl Square {
    pub fn new(
        freq: Knob,
        gain: Knob,
        duration: i32
    ) -> Self {
        Square { generative_waveform: GenerativeWaveform::new(
            freq,
            2,
            1,
            gain,
            true,
            duration
        ) }
    }
}

pub struct SquareState {
    gen_state: SoundData
}

const GEN_STATE: usize = 0;

impl SoundState for SquareState {
    fn get_sound_state(&self, key: usize) -> SoundData {
        match key {
            GEN_STATE => self.gen_state,
            _ => panic!("SquareState unknown key {}", key)
        }
    }
}

impl SoundSource for Square {
    fn init_state(&self) -> SoundData {
        Box::new(SquareState { gen_state: self.generative_waveform.init_state() })
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = state.downcast_mut::<MyData>().unwrap();
        self.generative_waveform.next_value(n, data.get_sound_state(GEN_STATE))
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