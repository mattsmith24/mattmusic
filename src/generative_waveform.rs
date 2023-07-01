pub mod generative_waveform {

// GenerativeWaveform is based on cpal example
// https://github.com/RustAudio/cpal/blob/master/examples/synth_tones.rs
// Apache License applies

use crate::traits::traits::SoundSource;
use crate::knob::knob::Knob;

pub struct GenerativeWaveform {
    sample_rate: f32,
    freq: Knob,
    harmonic_index_increment: i32,
    gain_exponent: f32,
    gain: Knob,
    duration: f32,
}
        
impl GenerativeWaveform {
    pub fn new(
        sample_rate: f32,
        freq: Knob,
        harmonic_index_increment: i32,
        gain_exponent: f32,
        gain: Knob,
        duration: f32
    ) -> Self {
        GenerativeWaveform{
            sample_rate: sample_rate,
            freq: freq,
            harmonic_index_increment: harmonic_index_increment,
            gain_exponent: gain_exponent,
            gain: gain,
            duration: duration,
        }
    }
    fn is_freq_above_nyquist(&self, freq: f32) -> bool {
        freq > self.sample_rate / 2.0
    }
    fn calculate_sine_output_from_freq(&self, freq: f32, t:f32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        (t * freq * two_pi).sin()
    }
}
        
impl SoundSource for GenerativeWaveform {
    fn next_value(&self, t: f32) -> (f32, f32) {
        if t > self.duration {
            (0.0, 0.0)
        } else {
            let mut output = 0.0;
            let freq = self.freq.next_value(t);
            let base_gain = self.gain.next_value(t);
            let mut i = 1;
            while !self.is_freq_above_nyquist(i as f32 * freq) {
                let gain = 1.0 / (i as f32).powf(self.gain_exponent);
                output += base_gain * gain * self.calculate_sine_output_from_freq(
                    freq * i as f32, t);
                i += self.harmonic_index_increment;
            }
            (output, output)
        }
    }

    fn duration(&self) -> f32 {
        self.duration
    }
}

}
