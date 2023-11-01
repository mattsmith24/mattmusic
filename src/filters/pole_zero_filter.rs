pub mod pole_zero_filter {

use crate::traits::traits::{SoundSource, DynSoundSource, DynComplexSoundSource, SoundData};
use crate::read_song::read_song::SongReader;
use crate::dc::dc::DC;
use crate::knob::knob::ComplexKnob;
use crate::multiply::multiply::Multiply;
use crate::filters::elementary_non_recirculating_filter::elementary_non_recirculating_filter::ComplexElementaryNonRecirculatingFilter;
use crate::filters::elementary_non_recirculating_filter_2nd_form::elementary_non_recirculating_filter_2nd_form::ComplexElementaryNonRecirculatingFilter2;
use crate::filters::elementary_recirculating_filter::elementary_recirculating_filter::ComplexElementaryRecirculatingFilter;
use crate::filters::real_to_complex::real_to_complex::RealToComplex;

#[derive(Clone)]
pub struct PoleZeroFilter {
    filter: DynComplexSoundSource,
}

impl PoleZeroFilter {
    pub fn new(input: DynSoundSource, normalize: DynSoundSource, poles: Vec<ComplexKnob>, zeros: Vec<ComplexKnob>,
        zero2s: Vec<ComplexKnob>) -> Self
    {
        let duration = input.duration();
        let mut normalize_input = Multiply::new();
        normalize_input.add(input, 0.0);
        normalize_input.add(normalize, 0.0);
        let mut filter: DynComplexSoundSource = Box::new(RealToComplex::new(Box::new(normalize_input),
            Box::new(DC::new(0.0, duration))));
        for pole in &poles {
            filter = Box::new(ComplexElementaryRecirculatingFilter::new(filter, pole.clone()));
        }
        for zero in &zeros {
            filter = Box::new(ComplexElementaryNonRecirculatingFilter::new(filter, zero.clone()));
        }
        for zero2 in &zero2s {
            filter = Box::new(ComplexElementaryNonRecirculatingFilter2::new(filter, zero2.clone()));
        }
        PoleZeroFilter {
            filter: filter,
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
        (output.0.re, output.1.re)
    }

    fn duration(&self) -> i32 {
        self.filter.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let normalize: DynSoundSource;
        if let Ok(normalize_dc) = params[1].parse::<f32>() {
            normalize = Box::new(DC::new(normalize_dc, input.duration()));
        } else {
            normalize = reader.get_sound(&params[1]);
        }
        let mut poles = Vec::<ComplexKnob>::new();
        let mut zeros = Vec::<ComplexKnob>::new();
        let mut zero2s = Vec::<ComplexKnob>::new();
        for idx in 2..params.len() {
            let param = &params[idx];
            let mut split = param.split(",");
            let param_type = split.next().unwrap();
            let point = reader.get_complex_knob(&(split.next().unwrap().to_owned()+","+split.next().unwrap()));
            match param_type {
                "pole" => poles.push(point),
                "zero" => zeros.push(point),
                "zero2" => zero2s.push(point),
                &_ => panic!("PoleZeroFilter::from_yaml() param type expected to be 'pole' or 'zero'. Got '{}'", param_type)
            }
        }
        Box::new(PoleZeroFilter::new(input, normalize, poles, zeros, zero2s))
    }
}

}