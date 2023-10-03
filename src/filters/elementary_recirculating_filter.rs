pub mod elementary_recirculating_filter {

use num::complex::Complex;

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

#[derive(Clone)]
pub struct ElementaryRecirculatingFilter {
    input: DynSoundSource,
    complex_gain: Complex<f32>
}

fn buffer_delayed_data(
    delayed_data: &mut Vec<(Complex<f32>,Complex<f32>)>,
    n: usize,
    sample: (Complex<f32>,Complex<f32>)
) {
    // This assumes that any previous data has been buffered already and
    // doesn't need updating. Also if we have data that has already been
    // buffered from a previous run through of this sound, then just do
    // nothing.
    if delayed_data.len() == n {
        delayed_data.push(sample);
    }
}

impl ElementaryRecirculatingFilter {
    pub fn new(input: DynSoundSource, complex_gain: Complex<f32>) -> Self {
        ElementaryRecirculatingFilter {
            input: input,
            complex_gain: complex_gain
        }
    }

    fn next_complex_value(&self, n:i32, data: &mut ElementaryRecirculatingFilterData) -> (Complex<f32>,Complex<f32>) {
        let real_input_value = self.input.next_value(n, &mut data.input_data);
        if n > 0 {
            // Handle case where previous values haven't been processed.
            // Sometimes sound sources higher up might start playing a sound
            // part way through in which case we won't have a previous sample
            // already buffered.
            if data.delayed_data.len() < n as usize {
                _ = self.next_complex_value(n - 1, data);
            }
            let input_value_0 = Complex::new(real_input_value.0, 0.0);
            let input_value_1 = Complex::new(real_input_value.1, 0.0);
            let delayed_input_value_0 = data.delayed_data[n as usize - 1].0;
            let delayed_input_value_1 = data.delayed_data[n as usize - 1].1;
            let output_0 = input_value_0 + delayed_input_value_0 * self.complex_gain;
            let output_1 = input_value_1 + delayed_input_value_1 * self.complex_gain;
            buffer_delayed_data(&mut data.delayed_data, n as usize, (output_0, output_1));
            (output_0, output_1)
        } else {
            let output = (Complex::new(real_input_value.0, 0.0), Complex::new(real_input_value.1, 0.0));
            if n == 0 {
                buffer_delayed_data(&mut data.delayed_data, 0, output);
            }
            output
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
        let mut data = &mut state.downcast_mut::<ElementaryRecirculatingFilterData>().unwrap();
        let output = self.next_complex_value(n, &mut data);
        (output.0.re, output.1.re)
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