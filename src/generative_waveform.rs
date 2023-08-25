pub mod generative_waveform {

// GenerativeWaveform is based on cpal example
// https://github.com/RustAudio/cpal/blob/master/examples/synth_tones.rs
// Apache License applies

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource};

use crate::knob::knob::Knob;

pub struct GenerativeWaveform {
    freq: Knob,
    harmonic_index_increment: i32,
    gain_exponent: i32,
    gain: Knob,
    lock_phase: bool,
    duration: i32,
    prev_freq: f32,
    phase_adjust: f32
}

impl GenerativeWaveform {
    pub fn new(
        freq: Knob,
        harmonic_index_increment: i32,
        gain_exponent: i32,
        gain: Knob,
        lock_phase: bool,
        duration: i32
    ) -> Self {
        GenerativeWaveform{
            freq: freq,
            harmonic_index_increment: harmonic_index_increment,
            gain_exponent: gain_exponent,
            gain: gain,
            lock_phase: lock_phase,
            duration: duration,
            prev_freq: 0.0,
            phase_adjust: 0.0
        }
    }
    fn is_freq_above_nyquist(&self, freq: f32) -> bool {
        freq > 0.5
    }
    fn calculate_sine_output_from_freq(&self, freq: f32, phase: f32, n:i32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        ((n as f32 * freq + phase) * two_pi).sin()
    }

    fn calculate_phase(&self, freq: f32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        let phase_div = (freq * n as f32) / two_pi;
        phase_div - phase_div.floor() + self.phase_adjust
    }

}

impl SoundSource for GenerativeWaveform {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        if n >= self.duration {
            (0.0, 0.0)
        } else {
            let mut output = 0.0;
            let base_gain = self.gain.next_value(n);
            let freq = self.freq.next_value(n);
            if self.lock_phase {
                if self.prev_freq == 0.0 {
                    self.prev_freq = freq;
                }
                let phase = self.calculate_phase(freq);
                let prev_phase = self.calculate_phase(self.prev_freq);
                // adjust the phase so that the new phase is the same as what
                // the phase would have been at the previous frequency
                self.phase_adjust -= phase - prev_phase;
                self.prev_freq = freq;
            }
            let mut i = 1;
            while !self.is_freq_above_nyquist(i as f32 * freq) {
                let gain = 1.0 / (i as f32).powf(self.gain_exponent as f32);
                output += gain * self.calculate_sine_output_from_freq(
                    freq * i as f32, self.phase_adjust * i as f32, n);
                i += self.harmonic_index_increment;
            }

            (output * base_gain, output * base_gain)
        }
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let freq = reader.get_knob(&params[0], 1.0 / reader.sample_rate as f32);
        let harmonic_index_increment = params[1].parse::<i32>().unwrap();
        let gain_exponent = params[2].parse::<i32>().unwrap();
        let gain = reader.get_knob(&params[3], 1.0);
        let lock_phase = params[4].parse::<bool>().unwrap();
        let duration = params[5].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(Self::new(freq, harmonic_index_increment, gain_exponent, gain, lock_phase, duration.round() as i32))
    }
}

}
