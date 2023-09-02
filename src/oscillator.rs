pub mod oscillator {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

use crate::knob::knob::Knob;

pub struct Oscillator {
    freq: Knob,
    phase: Knob,
    duration: i32,
}

impl Oscillator {
    pub fn new(
        freq: Knob,
        phase: Knob,
        duration: i32,
        ) -> Self {
        Oscillator{
            freq: freq,
            phase: phase,
            duration: duration
        }
    }
}

struct OscillatorData {
    freq_data: SoundData,
    phase_data: SoundData,
}

impl SoundSource for Oscillator {
    fn init_state(&self) -> SoundData {
        Box::new(OscillatorData {
            freq_data: self.freq.init_state(),
            phase_data: self.phase.init_state(),
        })
    }
    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        if n > self.duration {
            (0.0, 0.0)
        } else {
            let data = &mut state.downcast_mut::<OscillatorData>().unwrap();
            let val = ((n as f32 * self.freq.next_value(n, &mut data.freq_data)
                + self.phase.next_value(n, &mut data.phase_data))
                * 2.0 * std::f32::consts::PI).sin();
            (val, val)
        }
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let freq = reader.get_knob(&params[0], 1.0 / reader.sample_rate as f32);
        let phase = reader.get_knob(&params[1], 1.0 / reader.sample_rate as f32);
        let duration = params[2].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(Self::new(freq, phase, duration.round() as i32))
    }
}

}
