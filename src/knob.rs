pub mod knob {

use crate::traits::traits::{DynSoundSource, SoundData};
use crate::dc::dc::DC;

#[derive(Clone)]
pub struct Knob {
    input: DynSoundSource,
}

pub struct KnobData {
    input_data: SoundData
}

impl Knob {
    pub fn init_state(&self) -> SoundData {
        Box::new(KnobData{ input_data: self.input.init_state()})
    }

    pub fn new(input: DynSoundSource) -> Self {
        Knob { input: input }
    }

    pub fn dc(value: f32) -> Self {
        Knob::new(Box::new(DC::new(value, core::i32::MAX)))
    }

    pub fn next_value(&self, n: i32, state: &mut SoundData) -> f32 {
        let data = &mut state.downcast_mut::<KnobData>().unwrap();
        self.input.next_value(n, &mut data.input_data).0
    }
}

}