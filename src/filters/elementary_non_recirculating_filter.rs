pub mod elementary_non_recirculating_filter {

use num::complex::Complex;

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData,
    ComplexSoundSource, DynComplexSoundSource};
use crate::filters::real_to_complex::real_to_complex::RealToComplex;

#[derive(Clone)]
pub struct ComplexElementaryNonRecirculatingFilter {
    input: DynComplexSoundSource,
    gain: Complex<f32>
}

impl ComplexElementaryNonRecirculatingFilter {
    pub fn new(input: DynComplexSoundSource, gain: Complex<f32>) -> Self {
        ComplexElementaryNonRecirculatingFilter {
            input: input,
            gain: gain
        }
    }
}

struct ComplexElementaryNonRecirculatingFilterData {
    input_data: SoundData,
    delayed_input_data: SoundData,
}

impl ComplexSoundSource for ComplexElementaryNonRecirculatingFilter {
    fn init_state(&self) -> SoundData {
        Box::new(
            ComplexElementaryNonRecirculatingFilterData {
                input_data: self.input.init_state(),
                delayed_input_data: self.input.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (Complex<f32>, Complex<f32>) {
        let data = &mut state.downcast_mut::<ComplexElementaryNonRecirculatingFilterData>().unwrap();
        let input_value = self.input.next_value(n, &mut data.input_data);
        if n > 0 {
            let delayed_input_value = self.input.next_value(n - 1, &mut data.delayed_input_data);
            let output_0 = input_value.0 - delayed_input_value.0 * self.gain;
            let output_1 = input_value.1 - delayed_input_value.1 * self.gain;
            (output_0, output_1)
        } else {
            (input_value.0, input_value.1)
        }
    }

    fn duration(&self) -> i32 {
        self.input.duration()
    }
}


#[derive(Clone)]
pub struct ElementaryNonRecirculatingFilter {
    complex_filter: ComplexElementaryNonRecirculatingFilter,
}

impl ElementaryNonRecirculatingFilter {
    pub fn new(input: DynSoundSource, complex_gain: Complex<f32>) -> Self {
        let complex_input = Box::new(RealToComplex::new(input));
        ElementaryNonRecirculatingFilter {
            complex_filter: ComplexElementaryNonRecirculatingFilter::new(complex_input, complex_gain)
        }
    }
}

struct ElementaryNonRecirculatingFilterData {
    complex_filter_data: SoundData,
}

impl SoundSource for ElementaryNonRecirculatingFilter {
    fn init_state(&self) -> SoundData {
        Box::new(
            ElementaryNonRecirculatingFilterData {
                complex_filter_data: self.complex_filter.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<ElementaryNonRecirculatingFilterData>().unwrap();
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
        Box::new(ElementaryNonRecirculatingFilter::new(input, complex_gain))
    }
}

}