pub mod midi2freq {
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};
use crate::midi_notes::midi_notes::{midistr2freq, midi2freq};
use crate::read_song::read_song::SongReader;

pub struct Midi2Freq {
    freq: f32,
    duration: i32
}

impl Midi2Freq {
    pub fn new(freq: f32, duration: i32) -> Self {
        Midi2Freq { freq: freq, duration: duration }
    }
}

impl SoundSource for Midi2Freq {
    fn init_state(&self) -> SoundData {
        Box::new(0)
    }

    fn next_value(&self, n: i32, _state: &mut SoundData) -> (f32, f32) {
        if n < self.duration {
            (self.freq, self.freq)
        } else {
            (0.0, 0.0)
        }
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let value: f32;
        let char1 = params[0].chars().nth(0).unwrap();
        let note_range = 'A'..'H'; // doesn't include H
        if  note_range.contains(&char1) && params[0].len() <= 3 {
            value = midistr2freq(&params[0]) / reader.sample_rate as f32;
        } else {
            value = midi2freq(params[0].parse::<i8>().unwrap()) / reader.sample_rate as f32;
        }
        let duration = params[1].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(Self::new(value, duration.round() as i32))
    }
}


}