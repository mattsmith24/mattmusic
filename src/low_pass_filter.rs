pub mod low_pass_filter {

// References
// Tom Roelandts, How to Create a Simple Low-Pass Filter
// https://tomroelandts.com/articles/how-to-create-a-simple-low-pass-filter
//fc = 0.1
//b = 0.08
use std::collections::VecDeque;
use crate::traits::traits::{SoundSource, DynSoundSource};
use crate::knob::knob::Knob;

pub struct LowPassFilter {
    sample_rate: f32,
    frequency_cutoff_hz: Knob,
    transition_bandwidth: f32, // Transition band, as a fraction of the sampling rate (in (0, 0.5)).
    filter_length: usize,
    source: DynSoundSource
}

impl LowPassFilter {
    pub fn new(sample_rate: f32, frequency_cutoff_hz: Knob, transition_bandwidth_hz: f32, source: DynSoundSource) -> Self {
        let b = transition_bandwidth_hz / sample_rate; // Transition band, as a fraction of the sampling rate (in (0, 0.5)).
        let mut filter_length: usize = (4.0 / b).ceil() as usize; // number of samples in filter
        if filter_length % 2 == 0 {
            filter_length += 1  // Make sure that N is odd.
        }
        println!("filter_length = {} samples ({} seconds)", filter_length, filter_length as f32 / sample_rate);

        LowPassFilter {
            sample_rate: sample_rate,
            frequency_cutoff_hz: frequency_cutoff_hz,
            transition_bandwidth: b,
            filter_length: filter_length,
            source: source
        }
    }
}

fn sinc(x:f32) -> f32 {
    if x == 0.0 {
        1.0
    } else {
        (x * std::f32::consts::PI).sin() / (x * std::f32::consts::PI)
    }
}

fn vec_multiply_scalar(slice_a: &[f32], b:f32) -> Vec::<f32> {
    let mut output = Vec::<f32>::with_capacity(slice_a.len());
    for a in slice_a.iter() {
        output.push(a * b);
    }
    output
}

fn vec_multiply_vec(slice_a: &[f32], slice_b: &[f32]) -> Vec::<f32> {
    let mut output = Vec::<f32>::with_capacity(slice_a.len());
    for i in 0..slice_a.len() {
        output.push(slice_a[i] * slice_b[i]);
    }
    output
}

fn convolve(slice_a: &[f32], slice_b: &[f32]) -> Vec::<f32> {
    let mut output = Vec::<f32>::with_capacity(slice_a.len());
    for shift in 0..slice_a.len()+slice_b.len()-1 {
        let mut val = 0.0;
        for n in 0..slice_a.len() {
            if shift as i32 - n as i32 >= 0 && shift as i32 - (n as i32) < slice_b.len() as i32 {
                let a1 = slice_a[n];
                let b1 = slice_b[shift-n];
                val += a1 * b1;
            }
        }
        output.push(val);
    }
    output
}

impl SoundSource for LowPassFilter {
    fn next_value(&self, t: f32) -> (f32, f32) {

        let fc = self.frequency_cutoff_hz.next_value(t) / self.sample_rate; // Cutoff frequency as a fraction of the sampling rate (in (0, 0.5)).
        let b = self.transition_bandwidth;

        // Compute sinc filter.
        let mut h = Vec::<f32>::with_capacity(self.filter_length);
        for i in 0..self.filter_length {
            h.push(sinc(2.0 * fc * (i as i32 - (self.filter_length as i32 - 1) / 2) as f32));
        }

        // Compute Blackman window.
        let twopi = 2.0 * std::f32::consts::PI;
        let mut w = Vec::<f32>::with_capacity(self.filter_length);
        for i in 0..self.filter_length {
            w.push(0.42 - 0.5 * (twopi * i as f32 / (self.filter_length - 1) as f32).cos()
                + 0.08 * (2.0 * twopi * i as f32 / (self.filter_length - 1) as f32).cos());
        }

        // Multiply sinc filter by window.
        h = vec_multiply_vec(&h, &w);

        // Normalize to get unity gain.
        let sum_h: f32 = h.iter().sum();
        h = vec_multiply_scalar(&h, 1.0 / sum_h);

        let mut samples_left: Vec::<f32> = vec![0.0; self.filter_length];
        let mut samples_right: Vec::<f32> = vec![0.0; self.filter_length];
        for n in 0..self.filter_length {
            let t1 = t + (n as i32 + 1 - self.filter_length as i32) as f32 / self.sample_rate;
            if t1 > 0.0 {
                let s = (*self.source).next_value(t1);
                samples_left[n] = s.0;
                samples_right[n] = s.1;
            }
        }

        // convolution
        let output_left = convolve(&samples_left[..], &h[..]);
        let output_right = convolve(&samples_right[..], &h[..]);
        (output_left[self.filter_length as usize], output_right[self.filter_length as usize])
    }

    fn duration(&self) -> f32 {
        (*self.source).duration()
    }
}

}
