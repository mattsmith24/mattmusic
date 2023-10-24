pub mod elementary_recirculating_filter {

use num::complex::Complex;

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData,
    ComplexSoundSource, DynComplexSoundSource};
use crate::dc::dc::DC;
use crate::knob::knob::ComplexKnob;
use crate::filters::real_to_complex::real_to_complex::RealToComplex;

#[derive(Clone)]
pub struct ComplexElementaryRecirculatingFilter {
    input: DynComplexSoundSource,
    gain: ComplexKnob,
}

impl ComplexElementaryRecirculatingFilter {
    pub fn new(input: DynComplexSoundSource, gain: ComplexKnob) -> Self {
        ComplexElementaryRecirculatingFilter {
            input: input,
            gain: gain
        }
    }
}

struct ComplexElementaryRecirculatingFilterData {
    input_data: SoundData,
    prev_sample: (Complex<f32>, Complex<f32>),
    prev_sample_number: i32,
    gain_data: SoundData,
}

impl ComplexSoundSource for ComplexElementaryRecirculatingFilter {
    fn init_state(&self) -> SoundData {
        Box::new(
            ComplexElementaryRecirculatingFilterData {
                input_data: self.input.init_state(),
                prev_sample: (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)),
                prev_sample_number: -1,
                gain_data: self.gain.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (Complex<f32>, Complex<f32>) {
        let data = state.downcast_mut::<ComplexElementaryRecirculatingFilterData>().unwrap();
        let input_value = self.input.next_value(n, &mut data.input_data);
        if n > 0 {
            // Handle cases where the playback is out-of-order.
            // Sometimes sound sources higher up might start playing a sound
            // part way through in which case we won't have a previous sample
            // already buffered.
            if data.prev_sample_number == n {
                // We're repeating the previous sample
                data.prev_sample
            } else {
                if data.prev_sample_number < n - 1 {
                    // We've skipped forwards
                    for n1 in (data.prev_sample_number+1)..n {
                        _ = self.next_value(n1, state);
                    }
                }
                else if data.prev_sample_number > n {
                    // We've gone backwards
                    for n1 in 0..n {
                        _ = self.next_value(n1, state);
                    }
                }
                // We have to reborrow state because we might have just updated it in the above loop
                let data = state.downcast_mut::<ComplexElementaryRecirculatingFilterData>().unwrap();
                let gain = self.gain.next_value(n, &mut data.gain_data);
                let output_0 = input_value.0 + data.prev_sample.0 * gain;
                let output_1 = input_value.1 + data.prev_sample.1 * gain;
                data.prev_sample = (output_0, output_1);
                data.prev_sample_number = n;
                (output_0, output_1)
            }
        } else {
            let output = input_value;
            if n == 0 {
                data.prev_sample = output;
                data.prev_sample_number = n;
            }
            output
        }
    }

    fn duration(&self) -> i32 {
        self.input.duration()
    }

}


#[derive(Clone)]
pub struct ElementaryRecirculatingFilter {
    complex_filter: ComplexElementaryRecirculatingFilter,
}


impl ElementaryRecirculatingFilter {
    pub fn new(input: DynSoundSource, complex_gain: ComplexKnob) -> Self {
        let duration = input.duration();
        let complex_input = Box::new(RealToComplex::new(input, Box::new(DC::new(0.0, duration))));
        ElementaryRecirculatingFilter {
            complex_filter: ComplexElementaryRecirculatingFilter::new(complex_input, complex_gain),
        }
    }
}

struct ElementaryRecirculatingFilterData {
    complex_filter_data: SoundData,
}

impl SoundSource for ElementaryRecirculatingFilter {
    fn init_state(&self) -> SoundData {
        Box::new(
            ElementaryRecirculatingFilterData {
                complex_filter_data: self.complex_filter.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<ElementaryRecirculatingFilterData>().unwrap();
        let output = self.complex_filter.next_value(n, &mut data.complex_filter_data);
        (output.0.re, output.1.re)
    }

    fn duration(&self) -> i32 {
        self.complex_filter.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let complex_gain = reader.get_complex_knob(&params[1]);
        Box::new(ElementaryRecirculatingFilter::new(input, complex_gain))
    }
}

}