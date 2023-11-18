pub mod band_pass_filter {

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
pub struct BandPassFilter {
    filter: DynComplexSoundSource,
}

impl BandPassFilter {
    // Take a low pass filter with cutoff frequency at the desired bandwidth
    // β. Rotate it in the Z coordinates by the center frequency ω. Create a
    // conjugate pole to match it and multiply by normalising factor β ∗ (β +
    // 2ω)

    // centre and bandwidth should be in units of angular frequency: freq_hz * 2π / sample_rate.
    pub fn new(input: DynSoundSource, centre: DynSoundSource, bandwidth: DynSoundSource) -> Self
    {
        let duration = input.duration();
        // For band pass filters, an approximation for normalising is to
        // multiply by β ∗ (β + 2ω)
        let mut multiply_2ω = Multiply::new();
        multiply_2ω.add(centre.clone(), 0.0);
        multiply_2ω.add(Box::new(DC::new(2.0, duration)), 0.0);
        let mut add_β_plus_2ω = Mix::new();
        add_β_plus_2ω.add(bandwidth.clone());
        add_β_plus_2ω.add(Box::new(multiply_2ω));
        let mut normalize_input = Multiply::new();
        normalize_input.add(input, 0.0);
        normalize_input.add(bandwidth.clone(), 0.0);
        normalize_input.add(Box::new(add_β_plus_2ω), 0.0);
        let filter_input = RealToComplex::new(
            Box::new(TimeBox::new(duration, 0, Box::new(normalize_input))),
            Box::new(TimeBox::new(duration, 0, Box::new(DC::new(0.0, duration))))
        );
        // real pole: 1 - β
        let mut multiply_minus_β = Multiply::new();
        multiply_minus_β.add(bandwidth, 0.0);
        multiply_minus_β.add(Box::new(DC::new(-1.0, duration)), 0.0);
        let mut one_minus_β = Mix::new();
        one_minus_β.add(Box::new(DC::new(1.0, duration)));
        one_minus_β.add(Box::new(multiply_minus_β));
        // rotate pole by ω
        let pole = RealToComplex::new(Box::new(one_minus_β.clone()), centre.clone());
        let mut filter:DynComplexSoundSource = Box::new(
            ComplexElementaryRecirculatingFilter::new(Box::new(filter_input), ComplexKnob::new(Box::new(pole)))
        );
        // rotate pole by -ω
        let mut multiply_minus_ω = Multiply::new();
        multiply_minus_ω.add(centre, 0.0);
        multiply_minus_ω.add(Box::new(DC::new(-1.0, duration)), 0.0);
        let conjugate_pole = RealToComplex::new(Box::new(one_minus_β), Box::new(multiply_minus_ω));
        filter = Box::new(
            ComplexElementaryRecirculatingFilter::new(filter, ComplexKnob::new(Box::new(conjugate_pole)))
        );
        BandPassFilter {
            filter: filter,
        }
    }
}

struct BandPassFilterData {
    filter_data: SoundData,
}

impl SoundSource for BandPassFilter {
    fn init_state(&self) -> SoundData {
        Box::new(
            BandPassFilterData {
                filter_data: self.filter.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<BandPassFilterData>().unwrap();
        let output = self.filter.next_value(n, &mut data.filter_data);
        (output.0.re, output.1.re)
    }

    fn duration(&self) -> i32 {
        self.filter.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let centre = reader.get_sound(&params[1]);
        let bandwidth = reader.get_sound(&params[2]);
        Box::new(BandPassFilter::new(input, centre, bandwidth))
    }
}

}
