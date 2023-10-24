pub mod knob {

use num::Complex;

use crate::traits::traits::{DynSoundSource, SoundData, DynComplexSoundSource};
use crate::dc::dc::{DC, ComplexDC};

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

#[derive(Clone)]
pub struct ComplexKnob {
    input: DynComplexSoundSource,
}

pub struct ComplexKnobData {
    input_data: SoundData
}

impl ComplexKnob {
    pub fn init_state(&self) -> SoundData {
        Box::new(ComplexKnobData{ input_data: self.input.init_state()})
    }

    pub fn new(input: DynComplexSoundSource) -> Self {
        ComplexKnob { input: input }
    }

    pub fn dc(value: Complex<f32>) -> Self {
        ComplexKnob::new(Box::new(ComplexDC::new(value, core::i32::MAX)))
    }

    pub fn next_value(&self, n: i32, state: &mut SoundData) -> Complex<f32> {
        let data = &mut state.downcast_mut::<ComplexKnobData>().unwrap();
        self.input.next_value(n, &mut data.input_data).0
    }
}

}