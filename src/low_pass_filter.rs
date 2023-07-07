pub mod low_pass_filter {

// References
// Tom Roelandts, How to Create a Simple Low-Pass Filter
// https://tomroelandts.com/articles/how-to-create-a-simple-low-pass-filter
//fc = 0.1
//b = 0.08
use std::collections::VecDeque;
use crate::traits::traits::{SoundSource, DynSoundSource};

pub struct LowPassFilter {
    sample_rate: f32,
    filter: Vec::<f32>,
    source: DynSoundSource,
    samples_left: VecDeque::<f32>,
    samples_right: VecDeque::<f32>,
}

impl LowPassFilter {
    pub fn new(sample_rate: f32, frequency_cutoff_hz: f32, transition_bandwidth_hz: f32, source: DynSoundSource) -> Self {
        if transition_bandwidth_hz < frequency_cutoff_hz {
            panic!("transition_bandwidth_hz should be greater than frequency_cutoff_hz");
        }
        let b = transition_bandwidth_hz / sample_rate; // Transition band, as a fraction of the sampling rate (in (0, 0.5)).
        let mut filter_length: usize = (4.0 / b).ceil() as usize; // number of samples in filter
        if filter_length % 2 == 0 {
            filter_length += 1  // Make sure that N is odd.
        }
        //println!("filter_length = {} samples ({} seconds)", filter_length, filter_length as f32 / sample_rate);

        let fc = frequency_cutoff_hz / sample_rate; // Cutoff frequency as a fraction of the sampling rate (in (0, 0.5)).

        // Windowed Sinc Filter
        let twopi = 2.0 * std::f32::consts::PI;
        let len_minus_1 = (filter_length - 1) as f32;
        let mut h = Vec::<f32>::with_capacity(filter_length);
        for i in 0..filter_length {
            let n = i as f32;
            h.push(sinc(2.0 * fc * (n - len_minus_1 / 2.0))
                * (0.42 - 0.5 * (twopi * n / len_minus_1).cos()
                     + 0.08 * (2.0 * twopi * n / len_minus_1).cos()))
        }
        // Normalize to get unity gain.
        let sum_h: f32 = h.iter().sum();
        h = vec_multiply_scalar(&h, 1.0 / sum_h);

        // deques for samples (should be efficient way to store the last filter_length samples)
        let samples_left = VecDeque::<f32>::from(vec![0.0; filter_length]);
        let samples_right = VecDeque::<f32>::from(vec![0.0; filter_length]);

        LowPassFilter {
            sample_rate: sample_rate,
            filter: h,
            source: source,
            samples_left: samples_left,
            samples_right: samples_right
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

// Get value from tuple of slices as returned from DequeVec::as_slices()
// at index i.
fn slices_get(slices: (&[f32], &[f32]), i: usize) -> f32 {
    if i < slices.0.len() {
        slices.0[i]
    } else {
        slices.1[i - slices.0.len()]
    }
}

// This is continuous convolution so we only calculate the window for the current sample.
// slices_a: (&[f32], &[f32]) is designed to be the result of a DequeVec::as_slices()
// slices_a assumed to equal same length as slice_b.
// which returns a pair of slices which contain, in order, the contents of the deque
fn convolve(slices_a: (&[f32], &[f32]), slice_b: &[f32]) -> f32 {
    let slices_a_full_len = slices_a.0.len() + slices_a.1.len();
    let mut val = 0.0;
    for n in 0 .. slices_a_full_len {
        let a1 = slices_get(slices_a, n);
        let b1 = slice_b[slices_a_full_len - n - 1];
        val += a1 * b1;
    }
    val
}

impl SoundSource for LowPassFilter {
    fn next_value(&mut self, t: f32) -> (f32, f32) {
        let s = (*self.source).next_value(t);
        self.samples_left.push_back(s.0);
        self.samples_right.push_back(s.1);
        self.samples_left.pop_front();
        self.samples_right.pop_front();

        // convolution
        let output_left = convolve(self.samples_left.as_slices(), &self.filter[..]);
        let output_right = convolve(self.samples_right.as_slices(), &self.filter[..]);
        (output_left, output_right)
    }

    fn duration(&self) -> f32 {
        // add the filter delay to the duration
        (*self.source).duration() + (self.filter.len() - 1 / 2) as f32 / self.sample_rate
    }
}

}
