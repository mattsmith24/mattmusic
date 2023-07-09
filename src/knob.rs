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
        Knob { input: Box::new(DC::new(value, core::i32::MAX)) }
    }
    pub fn next_value(&mut self, n: i32) -> f32 {
        (*self.input).next_value(n).0
    }
}

}