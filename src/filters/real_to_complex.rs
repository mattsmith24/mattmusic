pub mod real_to_complex {

use num::complex::Complex;

use crate::traits::traits::{DynSoundSource, SoundData, ComplexSoundSource};

#[derive(Clone)]
pub struct RealToComplex {
    magnitude: DynSoundSource,
    angle: DynSoundSource,
}

impl RealToComplex {
    pub fn new(magnitude: DynSoundSource, angle: DynSoundSource) -> Self {
        RealToComplex {
            magnitude: magnitude,
            angle: angle,
        }
    }
}

struct RealToComplexData {
    magnitude_data: SoundData,
    angle_data: SoundData,
}

impl ComplexSoundSource for RealToComplex {
    fn init_state(&self) -> SoundData {
        Box::new(
            RealToComplexData {
                magnitude_data: self.magnitude.init_state(),
                angle_data: self.angle.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (Complex<f32>, Complex<f32>) {
        let data = &mut state.downcast_mut::<RealToComplexData>().unwrap();
        let magnitude = self.magnitude.next_value(n, &mut data.magnitude_data);
        let angle = self.angle.next_value(n, &mut data.angle_data);
        (Complex::from_polar(magnitude.0, angle.0), Complex::from_polar(magnitude.1, angle.1))
    }

    fn duration(&self) -> i32 {
        self.magnitude.duration()
    }
}


}
