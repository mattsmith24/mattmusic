pub mod db2amp {

use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};
use crate::read_song::read_song::SongReader;

#[derive(Clone)]
pub struct Db2Amp {
    amp: f32,
    duration: i32
}

impl Db2Amp {
    pub fn new(amp: f32, duration: i32) -> Self {
        Db2Amp { amp: amp, duration: duration }
    }
}

pub fn db2amp(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0) * 0.00001
}

#[allow(dead_code)]
pub fn amp2db(amp: f32) -> f32 {
    20.0 * (amp / 0.00001).log10()
}

impl SoundSource for Db2Amp {
    fn init_state(&self) -> SoundData {
        Box::new(0)
    }

    fn next_value(&self, n: i32, _state: &mut SoundData) -> (f32, f32) {
        if n < self.duration {
            (self.amp, self.amp)
        } else {
            (0.0, 0.0)
        }
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let value = db2amp(params[0].parse::<f32>().unwrap());
        let duration = params[1].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(Self::new(value, duration.round() as i32))
    }
}

}
