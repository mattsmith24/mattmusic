pub mod elementary_recirculating_filter {

use num::complex::Complex;

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

#[derive(Clone)]
pub struct ElementaryRecirculatingFilter {
    input: DynSoundSource,
    complex_gain: Complex<f32>
}

impl ElementaryRecirculatingFilter {
    pub fn new(input: DynSoundSource, complex_gain: Complex<f32>) -> Self {
        ElementaryRecirculatingFilter {
            input: input,
            complex_gain: complex_gain
        }
    }
}

struct ElementaryRecirculatingFilterData {
    input_data: SoundData,
    delayed_data: Vec<(Complex<f32>,Complex<f32>)>
}

impl SoundSource for ElementaryRecirculatingFilter {
    fn init_state(&self) -> SoundData {
        Box::new(
            ElementaryRecirculatingFilterData {
                input_data: self.input.init_state(),
                delayed_data: Vec::<(Complex<f32>,Complex<f32>)>::new(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<ElementaryRecirculatingFilterData>().unwrap();
        if n > 0 {
            let real_input_value = self.input.next_value(n, &mut data.input_data);
            let input_value_0 = Complex::new(real_input_value.0, 0.0);
            let input_value_1 = Complex::new(real_input_value.1, 0.0);
            let delayed_input_value_0 = data.delayed_data[n as usize - 1].0;
            let delayed_input_value_1 = data.delayed_data[n as usize - 1].1;
            let output_0 = input_value_0 + delayed_input_value_0 * self.complex_gain;
            let output_1 = input_value_1 + delayed_input_value_1 * self.complex_gain;
            data.delayed_data.push((output_0, output_1));
            (output_0.re, output_1.re)
        } else {
            if n == 0 {
                data.delayed_data.push((Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)));
            }
            (0.0, 0.0)
        }
    }

    fn duration(&self) -> i32 {
        self.input.duration()
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