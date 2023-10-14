pub mod elementary_recirculating_filter {

use num::complex::Complex;

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData,
    ComplexSoundSource, DynComplexSoundSource};
use crate::filters::real_to_complex::real_to_complex::RealToComplex;

#[derive(Clone)]
pub struct ComplexElementaryRecirculatingFilter {
    input: DynComplexSoundSource,
    gain: Complex<f32>
}

fn buffer_recirculating_data(
    recirculating_data: &mut Vec<(Complex<f32>,Complex<f32>)>,
    n: usize,
    sample: (Complex<f32>,Complex<f32>)
) {
    // This assumes that any previous data has been buffered already and
    // doesn't need updating. Also if we have data that has already been
    // buffered from a previous run through of this sound, then just do
    // nothing.
    if recirculating_data.len() == n {
        recirculating_data.push(sample);
    }
}

impl ComplexElementaryRecirculatingFilter {
    pub fn new(input: DynComplexSoundSource, gain: Complex<f32>) -> Self {
        ComplexElementaryRecirculatingFilter {
            input: input,
            gain: gain
        }
    }
}

struct ComplexElementaryRecirculatingFilterData {
    input_data: SoundData,
    recirculating_data: Vec<(Complex<f32>,Complex<f32>)>
}

impl ComplexSoundSource for ComplexElementaryRecirculatingFilter {
    fn init_state(&self) -> SoundData {
        Box::new(
            ComplexElementaryRecirculatingFilterData {
                input_data: self.input.init_state(),
                recirculating_data: Vec::<(Complex<f32>,Complex<f32>)>::new(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (Complex<f32>, Complex<f32>) {
        let data = state.downcast_mut::<ComplexElementaryRecirculatingFilterData>().unwrap();
        let input_value = self.input.next_value(n, &mut data.input_data);
        if n > 0 {
            // Handle case where previous values haven't been processed.
            // Sometimes sound sources higher up might start playing a sound
            // part way through in which case we won't have a previous sample
            // already buffered.
            if data.recirculating_data.len() < n as usize {
                for n1 in (data.recirculating_data.len() as i32)..n {
                    _ = self.next_value(n1, state);
                }
            }
            // We have to reborrow state because we might have just updated it in the above loop
            let data = state.downcast_mut::<ComplexElementaryRecirculatingFilterData>().unwrap();
            let delayed_input_value = data.recirculating_data[n as usize - 1];
            let output_0 = input_value.0 + delayed_input_value.0 * self.gain;
            let output_1 = input_value.1 + delayed_input_value.1 * self.gain;
            buffer_recirculating_data(&mut data.recirculating_data, n as usize, (output_0, output_1));
            (output_0, output_1)
        } else {
            let output = input_value;
            if n == 0 {
                buffer_recirculating_data(&mut data.recirculating_data, 0, output);
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
    pub fn new(input: DynSoundSource, complex_gain: Complex<f32>) -> Self {
        let complex_input = Box::new(RealToComplex::new(input));
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
        let complex_gain_magnitude = params[1].parse::<f32>().unwrap();
        let complex_gain_angle = params[2].parse::<f32>().unwrap();
        let complex_gain = Complex::from_polar(complex_gain_magnitude, complex_gain_angle);
        Box::new(ElementaryRecirculatingFilter::new(input, complex_gain))
    }
}

}