pub mod sine {

use crate::read_song::read_song::{YAMLFormat, get_knob};
use crate::traits::traits::{SoundSource, DynSoundSource};

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

    fn from_yaml(params: &Vec::<String>, yaml: &YAMLFormat, sample_rate: i32) -> DynSoundSource {
        let freq = get_knob(&params[0], 1.0 / sample_rate as f32, yaml, sample_rate);
        let strength = get_knob(&params[1], 1.0, yaml, sample_rate);
        let duration = params[2].parse::<f32>().unwrap() * sample_rate as f32;
        Box::new(Sine::new(freq, strength, duration.round() as i32))
    }
}

}