pub mod generative_waveform {

// GenerativeWaveform is based on cpal example
// https://github.com/RustAudio/cpal/blob/master/examples/synth_tones.rs
// Apache License applies

use crate::traits::traits::SoundSource;
use crate::knob::knob::Knob;

pub struct GenerativeWaveform {
    freq: Knob,
    harmonic_index_increment: i32,
    gain_exponent: i32,
    gain: Knob,
    duration: i32,
    // next_clock: u32,
    // period_lock: u32,
    // phase_adjusn: i32
}

impl GenerativeWaveform {
    pub fn new(
        freq: Knob,
        harmonic_index_increment: i32,
        gain_exponent: i32,
        gain: Knob,
        duration: i32
    ) -> Self {
        GenerativeWaveform{
            freq: freq,
            harmonic_index_increment: harmonic_index_increment,
            gain_exponent: gain_exponent,
            gain: gain,
            duration: duration,
            // next_clock: 0,
            // period_lock: 0,
            // phase_adjust: 0.0
        }
    }
    fn is_freq_above_nyquist(&self, freq: f32) -> bool {
        freq > 0.5
    }
    fn calculate_sine_output_from_freq(&self, freq: f32, n:i32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        (n as f32 * freq * two_pi).sin()
    }
}

impl SoundSource for GenerativeWaveform {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        if n > self.duration {
            (0.0, 0.0)
        } else {
            let mut output = 0.0;
            let freq = self.freq.next_value(n);
            let base_gain = self.gain.next_value(n);
            let mut i = 1;
            while !self.is_freq_above_nyquist(i as f32 * freq) {
                let gain = 1.0 / (i as f32).powf(self.gain_exponent as f32);
                output += base_gain * gain * self.calculate_sine_output_from_freq(
                    freq * i as f32, n);
                i += self.harmonic_index_increment;
            }
            (output, output)
        }
    }

    fn duration(&self) -> i32 {
        self.duration
    }
}

}
