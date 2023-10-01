pub mod pulse_train {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

use crate::knob::knob::Knob;

#[derive(Clone)]
pub struct PulseTrain {
    freq: Knob, // Frequency as fraction of sample rate
    duty: Knob, // Ratio of on to off in each period. 0.5 is square wave
    duration: i32,
}

impl PulseTrain {
    pub fn new(freq: Knob, duty: Knob, duration: i32) -> Self {
        PulseTrain {
            freq: freq,
            duty: duty,
            duration: duration,
        }
    }

    // Phase is a fraction of how far through the cycle we are. It doesn't need
    // to relate to pi since we aren't using a sine function anywhere
    fn calculate_phase(&self, freq: f32, n:i32, phase_adjust: f32) -> f32 {
        let phase_div = freq * n as f32;
        let mut phase = phase_div - phase_div.floor() + phase_adjust;
        while phase < 0.0 {
            phase += 1.0;
        }
        while phase > 1.0 {
            phase -= 1.0;
        }
        phase
    }
}

struct PulseTrainData {
    prev_freq: f32,
    phase_adjust: f32,
    freq_data: SoundData,
    duty_data: SoundData,
}

impl SoundSource for PulseTrain {
    fn init_state(&self) -> SoundData {
        Box::new(PulseTrainData {
            prev_freq: 0.0,
            phase_adjust: 0.0,
            freq_data: self.freq.init_state(),
            duty_data: self.duty.init_state(),
        })
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        if n >= self.duration {
            (0.0, 0.0)
        } else {
            let mut data = state.downcast_mut::<PulseTrainData>().unwrap();
            let duty = self.duty.next_value(n, &mut data.duty_data);
            let freq = self.freq.next_value(n, &mut data.freq_data);
            let mut phase_adjust = data.phase_adjust;
            if data.prev_freq != 0.0 {
                let phase = self.calculate_phase(freq, n, data.phase_adjust);
                let prev_phase = self.calculate_phase(data.prev_freq, n, data.phase_adjust);
                // adjust the phase so that the new phase is the same as what
                // the phase would have been at the previous frequency
                phase_adjust -= phase - prev_phase;
                while phase_adjust < 0.0 {
                    phase_adjust += 1.0;
                }
                while phase_adjust > 1.0 {
                    phase_adjust -= 1.0;
                }
            }
            data.phase_adjust = phase_adjust;
            data.prev_freq = freq;
            // Instead of a sine function, we just calculate how far through the
            // cycle we are compare to the duty. The signum function will be 1
            // if we are before the duty point or -1 if we are past it. We then
            // adjust the range of that to be between 0 and 1
            let output = (duty - self.calculate_phase(freq, n, phase_adjust)).signum() * 0.5 + 0.5;
            (output, output)
        }
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let freq = reader.get_knob(&params[0], 1.0 / reader.sample_rate as f32);
        let duty = reader.get_knob(&params[1], 1.0);
        let duration = params[2].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(Self::new(freq, duty, duration.round() as i32))
    }


}


}