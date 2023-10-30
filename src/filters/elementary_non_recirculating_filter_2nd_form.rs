pub mod elementary_non_recirculating_filter_2nd_form {

use num::complex::Complex;

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData,
    ComplexSoundSource, DynComplexSoundSource};
use crate::dc::dc::DC;
use crate::knob::knob::ComplexKnob;
use crate::filters::real_to_complex::real_to_complex::RealToComplex;

// The second form of non recirculating filter multiplies the input by the
// conjugate of the gain to get the same result as for the first form only this
// version changes the phase of the signal.
//
//
//                           Input
//                             |
//                   +---------+
//                   |         |
//                   |         V
//                   V        +-+
//            _     +-+       | |
//            Q --->|X|       | | delay = 1
//                  +-+       | |
//                   |        +-+
//                   |         |
//                   V         V
//                 +-------------+
//                 |      -      |
//                 +-------------+
//                        |
//                        V
//                      Output

#[derive(Clone)]
pub struct ComplexElementaryNonRecirculatingFilter2 {
    input: DynComplexSoundSource,
    gain: ComplexKnob,
}

impl ComplexElementaryNonRecirculatingFilter2 {
    pub fn new(input: DynComplexSoundSource, gain: ComplexKnob) -> Self {
        ComplexElementaryNonRecirculatingFilter2 {
            input: input,
            gain: gain
        }
    }
}

struct ComplexElementaryNonRecirculatingFilter2Data {
    input_data: SoundData,
    delayed_input_data: SoundData,
    gain_data: SoundData,
}

impl ComplexSoundSource for ComplexElementaryNonRecirculatingFilter2 {
    fn init_state(&self) -> SoundData {
        Box::new(
            ComplexElementaryNonRecirculatingFilter2Data {
                input_data: self.input.init_state(),
                delayed_input_data: self.input.init_state(),
                gain_data: self.gain.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (Complex<f32>, Complex<f32>) {
        let data = &mut state.downcast_mut::<ComplexElementaryNonRecirculatingFilter2Data>().unwrap();
        let input_value = self.input.next_value(n, &mut data.input_data);
        if n > 0 {
            let delayed_input_value = self.input.next_value(n - 1, &mut data.delayed_input_data);
            let gain = self.gain.next_value(n, &mut data.gain_data);
            let output_0 = input_value.0 * gain.conj() - delayed_input_value.0;
            let output_1 = input_value.1 * gain.conj() - delayed_input_value.1;
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
pub struct ElementaryNonRecirculatingFilter2 {
    complex_filter: ComplexElementaryNonRecirculatingFilter2,
}

impl ElementaryNonRecirculatingFilter2 {
    pub fn new(input: DynSoundSource, complex_gain: ComplexKnob) -> Self {
        let duration = input.duration();
        let complex_input = Box::new(RealToComplex::new(input, Box::new(DC::new(0.0, duration))));
        ElementaryNonRecirculatingFilter2 {
            complex_filter: ComplexElementaryNonRecirculatingFilter2::new(complex_input, complex_gain)
        }
    }
}

struct ElementaryNonRecirculatingFilter2Data {
    complex_filter_data: SoundData,
}

impl SoundSource for ElementaryNonRecirculatingFilter2 {
    fn init_state(&self) -> SoundData {
        Box::new(
            ElementaryNonRecirculatingFilter2Data {
                complex_filter_data: self.complex_filter.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<ElementaryNonRecirculatingFilter2Data>().unwrap();
        let output = self.complex_filter.next_value(n, &mut data.complex_filter_data);
        (output.0.re, output.1.re)
    }

    fn duration(&self) -> i32 {
        self.complex_filter.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let complex_gain = reader.get_complex_knob(&params[1]);
        Box::new(ElementaryNonRecirculatingFilter2::new(input, complex_gain))
    }
}

}