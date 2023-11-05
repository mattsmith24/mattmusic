pub mod allpass_filter {

use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};
use crate::read_song::read_song::SongReader;
use crate::knob::knob::ComplexKnob;
use crate::dc::dc::DC;
use crate::filters::pole_zero_filter::pole_zero_filter::PoleZeroFilter;

#[derive(Clone)]
pub struct AllpassFilter {
    filter: PoleZeroFilter,
}

impl AllpassFilter {
    pub fn new(input: DynSoundSource, gain: ComplexKnob) -> Self {
        let mut poles = Vec::<ComplexKnob>::new();
        poles.push(gain.clone());
        let zeros = Vec::<ComplexKnob>::new();
        let mut zero2s = Vec::<ComplexKnob>::new();
        zero2s.push(gain.clone());
        let duration = input.duration();
        let normalize_dc = Box::new(DC::new(1.0, duration));
        AllpassFilter {
            filter: PoleZeroFilter::new(input, normalize_dc, poles, zeros, zero2s),
        }
    }
}

struct AllpassFilterData {
    filter_data: SoundData,
}

impl SoundSource for AllpassFilter {
    fn init_state(&self) -> SoundData {
        Box::new(
            AllpassFilterData {
                filter_data: self.filter.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<AllpassFilterData>().unwrap();
        self.filter.next_value(n, &mut data.filter_data)
    }

    fn duration(&self) -> i32 {
        self.filter.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let gain = reader.get_complex_knob(&params[1]);
        Box::new(AllpassFilter::new(input, gain))
    }
}

}