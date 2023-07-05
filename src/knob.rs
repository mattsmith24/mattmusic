pub mod knob {

use crate::traits::traits::DynSoundSource;
use crate::dc::dc::DC;

pub struct Knob {
    input: DynSoundSource
}

impl Knob {
    pub fn new(input: DynSoundSource) -> Self {
        Knob { input: input }
    }
    pub fn dc(value: f32) -> Self {
        Knob { input: Box::new(DC::new(value, core::f32::MAX)) }
    }
    pub fn next_value(&mut self, t: f32) -> f32 {
        (*self.input).next_value(t).0
    }
}

}