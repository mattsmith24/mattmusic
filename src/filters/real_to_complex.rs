pub mod real_to_complex {

use num::complex::Complex;

use crate::traits::traits::{DynSoundSource, SoundData, ComplexSoundSource};

#[derive(Clone)]
pub struct RealToComplex {
    input: DynSoundSource,
}

impl RealToComplex {
    pub fn new(input: DynSoundSource) -> Self {
        RealToComplex { input: input }
    }
}

struct RealToComplexData {
    input_data: SoundData,
}

impl ComplexSoundSource for RealToComplex {
    fn init_state(&self) -> SoundData {
        Box::new(
            RealToComplexData {
                input_data: self.input.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (Complex<f32>, Complex<f32>) {
        let data = &mut state.downcast_mut::<RealToComplexData>().unwrap();
        let input_value = self.input.next_value(n, &mut data.input_data);
        (Complex::new(input_value.0, 0.0), Complex::new(input_value.1, 0.0))
    }

    fn duration(&self) -> i32 {
        self.input.duration()
    }
}


}
