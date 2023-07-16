pub mod generative_waveform {

// GenerativeWaveform is based on cpal example
// https://github.com/RustAudio/cpal/blob/master/examples/synth_tones.rs
// Apache License applies

use crate::read_song::read_song::YAMLFormat;
use crate::traits::traits::{SoundSource, DynSoundSource};

use crate::knob::knob::Knob;

pub struct GenerativeWaveform {
    freq: Knob,
    harmonic_index_increment: i32,
    gain_exponent: i32,
    gain: Knob,
    duration: i32,
    next_clock: i32,
    freq_lock: f32,
    phase_adjust: f32
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
            next_clock: 0,
            freq_lock: 0.0,
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
}

impl SoundSource for GenerativeWaveform {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        if n > self.duration {
            (0.0, 0.0)
        } else {
            let mut output = 0.0;
            let base_gain = self.gain.next_value(n);
            // Use a clock to lock the frequency for each cycle. This prevents phase artifacts
            // resulting from a constantly varying frequency input.
            // n == 0 is used to reset when replaying the sound
            if n == 0 {
                self.next_clock = 0;
            }
            if n >= self.next_clock {
                let freq = self.freq.next_value(n);
                // calc period from desired frequency
                self.next_clock += (1.0 / freq).round() as i32;
                // lock the frequency until the next clock
                self.freq_lock = freq;
                // subtract any phase at this value of n so that we start the waveform at 0
                self.phase_adjust = -freq * n as f32;
            }
            let mut i = 1;
            while !self.is_freq_above_nyquist(i as f32 * self.freq_lock) {
                let gain = 1.0 / (i as f32).powf(self.gain_exponent as f32);
                output += base_gain * gain * self.calculate_sine_output_from_freq(
                    self.freq_lock * i as f32, self.phase_adjust * i as f32, n);
                i += self.harmonic_index_increment;
            }
            (output, output)
        }
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(params: &Vec::<String>, yaml: &YAMLFormat, sample_rate: i32) -> DynSoundSource {
        use crate::dc::dc::DC;
        todo!();
        Box::new(Self::new(Knob::dc(0.0), 0, 0, Knob::dc(0.0), 0))
    }
}

}
