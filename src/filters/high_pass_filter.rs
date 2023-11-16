pub mod high_pass_filter {

use num::complex::Complex;

use crate::traits::traits::{SoundSource, DynSoundSource, DynComplexSoundSource, SoundData};
use crate::read_song::read_song::SongReader;
use crate::dc::dc::DC;
use crate::knob::knob::ComplexKnob;
use crate::mix::mix::Mix;
use crate::multiply::multiply::Multiply;
use crate::time_box::time_box::TimeBox;
use crate::filters::elementary_non_recirculating_filter::elementary_non_recirculating_filter::ComplexElementaryNonRecirculatingFilter;
use crate::filters::elementary_recirculating_filter::elementary_recirculating_filter::ComplexElementaryRecirculatingFilter;
use crate::filters::real_to_complex::real_to_complex::RealToComplex;

#[derive(Clone)]
pub struct HighPassFilter {
    filter: DynComplexSoundSource,
}

impl HighPassFilter {
    // cutoff should be in units of angular frequency: ω = freq_hz * 2π / sample_rate
    pub fn new(input: DynSoundSource, cutoff: DynSoundSource) -> Self
    {
        let duration = input.duration();
        // Calculate 1 - ω for use as normalisation and pole
        let cutoff_timebox = TimeBox::new(duration, 0, cutoff);
        let mut negative_cutoff = Multiply::new();
        negative_cutoff.add(Box::new(cutoff_timebox), 0.0);
        negative_cutoff.add(Box::new(DC::new(-1.0, duration)), 0.0);
        let mut one_minus_omega = Mix::new();
        one_minus_omega.add(Box::new(DC::new(1.0, duration)));
        one_minus_omega.add(Box::new(negative_cutoff));
        // For high pass filters, an approximation for normalising is to multiply
        // by 1 - ω assuming it's relatively small
        let mut normalize_input = Multiply::new();
        normalize_input.add(input, 0.0);
        normalize_input.add(Box::new(one_minus_omega.clone()), 0.0);
        let filter_input = Box::new(RealToComplex::new(Box::new(normalize_input),
            Box::new(DC::new(0.0, duration))));
        // pole = 1 - ω
        let pole = Box::new(RealToComplex::new(Box::new(one_minus_omega), Box::new(DC::new(0.0, duration))));
        let mut filter:DynComplexSoundSource = Box::new(
            ComplexElementaryRecirculatingFilter::new(filter_input, ComplexKnob::new(pole))
        );
        // zero = 1
        filter = Box::new(ComplexElementaryNonRecirculatingFilter::new(filter,
            ComplexKnob::dc(Complex::<f32>::new(1.0, 0.0))));
        HighPassFilter {
            filter: filter,
        }
    }
}

struct HighPassFilterData {
    filter_data: SoundData,
}

impl SoundSource for HighPassFilter {
    fn init_state(&self) -> SoundData {
        Box::new(
            HighPassFilterData {
                filter_data: self.filter.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<HighPassFilterData>().unwrap();
        let output = self.filter.next_value(n, &mut data.filter_data);
        (output.0.re, output.1.re)
    }

    fn duration(&self) -> i32 {
        self.filter.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let cutoff = reader.get_sound(&params[1]);
        Box::new(HighPassFilter::new(input, cutoff))
    }
}

}
