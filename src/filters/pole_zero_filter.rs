pub mod pole_zero_filter {

use num::complex::Complex;

use crate::traits::traits::{SoundSource, DynSoundSource, DynComplexSoundSource, SoundData};
use crate::read_song::read_song::SongReader;
use crate::filters::elementary_non_recirculating_filter::elementary_non_recirculating_filter::ComplexElementaryNonRecirculatingFilter;
use crate::filters::elementary_recirculating_filter::elementary_recirculating_filter::ComplexElementaryRecirculatingFilter;
use crate::filters::real_to_complex::real_to_complex::RealToComplex;

#[derive(Clone)]
pub struct PoleZeroFilter {
    filter: DynComplexSoundSource,
    normalize: f32,
}

impl PoleZeroFilter {
    pub fn new(input: DynSoundSource, poles: Vec<Complex<f32>>, zeros: Vec<Complex<f32>>, normalize: f32) -> Self
    {
        let mut filter: DynComplexSoundSource = Box::new(RealToComplex::new(input));
        for pole in &poles {
            filter = Box::new(ComplexElementaryRecirculatingFilter::new(filter, pole.clone()));
        }
        for zero in &zeros {
            filter = Box::new(ComplexElementaryNonRecirculatingFilter::new(filter, zero.clone()));
        }
        PoleZeroFilter {
            filter: filter,
            normalize: normalize,
        }
    }
}

struct PoleZeroFilterData {
    filter_data: SoundData,
}

impl SoundSource for PoleZeroFilter {
    fn init_state(&self) -> SoundData {
        Box::new(
            PoleZeroFilterData {
                filter_data: self.filter.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<PoleZeroFilterData>().unwrap();
        let output = self.filter.next_value(n, &mut data.filter_data);
        (output.0.re * self.normalize, output.1.re * self.normalize)
    }

    fn duration(&self) -> i32 {
        self.filter.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let normalize = params[1].parse::<f32>().unwrap();
        let mut poles = Vec::<Complex::<f32>>::new();
        let mut zeros = Vec::<Complex::<f32>>::new();
        for idx in 2..params.len() {
            let param = &params[idx];
            let parts: Vec<_> = param.split(" ").collect();
            if parts.len() != 3 {
                panic!("PoleZeroFilter::from_yaml() param expected to be <pole|zero> <magnitude> <angle>. Instead got '{}'",
                    param);
            }
            let param_type = parts[0];
            let magnitude = parts[1].parse::<f32>().unwrap();
            let angle = parts[2].parse::<f32>().unwrap();
            let complex = Complex::from_polar(magnitude, angle);
            match param_type {
                "pole" => poles.push(complex),
                "zero" => zeros.push(complex),
                &_ => panic!("PoleZeroFilter::from_yaml() param type expected to be 'pole' or 'zero'. Got '{}'", param_type)
            }
        }
        Box::new(PoleZeroFilter::new(input, poles, zeros, normalize))
    }
}

}