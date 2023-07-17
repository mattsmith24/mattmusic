pub mod square {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource};

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
            duration
        ) }
    }
}

impl SoundSource for Square {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        self.generative_waveform.next_value(n)
    }

    fn duration(&self) -> i32 {
        self.generative_waveform.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        use crate::dc::dc::DC;
        todo!();
        Box::new(Self::new(Knob::dc(0.0), Knob::dc(0.0), 0))
    }
}


}