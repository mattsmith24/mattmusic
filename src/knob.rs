pub mod knob {

use crate::traits::traits::DynSoundSource;
use crate::dc::dc::DC;

pub struct Knob {
    input: DynSoundSource,
    debug: bool
}

impl Knob {
    pub fn new(input: DynSoundSource) -> Self {
        Knob { input: input, debug: false }
    }
    pub fn dc(value: f32) -> Self {
        Knob { input: Box::new(DC::new(value, core::f32::MAX)), debug: false }
    }
    pub fn next_value(&self, t: f32) -> f32 {
        let val = (*self.input).next_value(t).0;
        if self.debug && (t-(10.0*t).round()/10.0).abs() < 0.000001 {
            println!("knob value: {}, {}", t, val)
        }
        val
    }
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
}

}