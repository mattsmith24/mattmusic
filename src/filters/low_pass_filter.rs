pub mod low_pass_filter {

use crate::traits::traits::{SoundSource, DynSoundSource, DynComplexSoundSource, SoundData};
use crate::read_song::read_song::SongReader;
use crate::dc::dc::DC;
use crate::knob::knob::ComplexKnob;
use crate::mix::mix::Mix;
use crate::multiply::multiply::Multiply;
use crate::time_box::time_box::TimeBox;
use crate::filters::elementary_recirculating_filter::elementary_recirculating_filter::ComplexElementaryRecirculatingFilter;
use crate::filters::real_to_complex::real_to_complex::RealToComplex;

#[derive(Clone)]
pub struct LowPassFilter {
    filter: DynComplexSoundSource,
}

impl LowPassFilter {
    // cutoff should be in units of angular frequency: freq_hz * 2π / sample_rate
    pub fn new(input: DynSoundSource, cutoff: DynSoundSource) -> Self
    {
        let duration = input.duration();
        // For low pass filters, an approximation for normalising is to multiply
        // by the cutoff freq assuming it's relatively small
        let mut normalize_input = Multiply::new();
        normalize_input.add(input, 0.0);
        let cutoff_timebox = TimeBox::new(duration, 0, cutoff.clone());
        normalize_input.add(Box::new(cutoff_timebox), 0.0);

        let filter_input = Box::new(RealToComplex::new(Box::new(normalize_input),
            Box::new(DC::new(0.0, duration))));
        // pole = 1 - ω
        let mut negative_cutoff = Multiply::new();
        negative_cutoff.add(cutoff, 0.0);
        negative_cutoff.add(Box::new(DC::new(-1.0, duration)), 0.0);
        let mut pole_real = Mix::new();
        pole_real.add(Box::new(DC::new(1.0, duration)));
        pole_real.add(Box::new(negative_cutoff));
        let pole = Box::new(RealToComplex::new(Box::new(pole_real), Box::new(DC::new(0.0, duration))));
        let filter = Box::new(ComplexElementaryRecirculatingFilter::new(filter_input, ComplexKnob::new(pole)));
        LowPassFilter {
            filter: filter,
        }
    }
}

struct LowPassFilterData {
    filter_data: SoundData,
}

impl SoundSource for LowPassFilter {
    fn init_state(&self) -> SoundData {
        Box::new(
            LowPassFilterData {
                filter_data: self.filter.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<LowPassFilterData>().unwrap();
        let output = self.filter.next_value(n, &mut data.filter_data);
        (output.0.re, output.1.re)
    }

    fn duration(&self) -> i32 {
        self.filter.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let cutoff = reader.get_sound(&params[1]);
        Box::new(LowPassFilter::new(input, cutoff))
    }
}

}
