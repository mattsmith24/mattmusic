pub mod multiply {

use crate::traits::traits::{SoundSource, DynSoundSource};

pub struct MultiplyInput {
    source: DynSoundSource,
    offset: f32
}

pub struct Multiply {
    inputs: Vec<MultiplyInput>
}

impl Multiply {
    pub fn new() -> Self {
        Multiply{ inputs: Vec::<MultiplyInput>::new() }
    }

    pub fn add(&mut self, source: DynSoundSource, offset: f32) -> &mut Multiply {
        self.inputs.push(MultiplyInput { source:source, offset: offset });
        self
    }
}

impl SoundSource for Multiply {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        let mut res1: f32 = 1.0;
        let mut res2: f32 = 1.0;
        for minput in self.inputs.iter_mut() {
            let (v1, v2) = (*minput.source).next_value(n);
            res1 *= (v1 + minput.offset);
            res2 *= (v2 + minput.offset);
        }
        (res1, res2)
    }
    fn duration(&self) -> i32 {
        let mut duration: i32 = 0;
        for minput in self.inputs.iter() {
            duration = duration.max((*minput.source).duration());
        }
        duration
    }
}

}