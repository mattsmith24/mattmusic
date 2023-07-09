pub mod multiply {

use crate::traits::traits::{SoundSource, DynSoundSource};

pub struct Multiply {
    sources: Vec<DynSoundSource>
}

impl Multiply {
    pub fn new() -> Self {
        Multiply{ sources: Vec::<DynSoundSource>::new() }
    }

    pub fn add(&mut self, source: DynSoundSource) -> &mut Multiply {
        self.sources.push(source);
        self
    }
}

impl SoundSource for Multiply {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        let mut res1: f32 = 1.0;
        let mut res2: f32 = 1.0;
        for source in self.sources.iter_mut() {
            let (v1, v2) = (*source).next_value(n);
            res1 *= v1;
            res2 *= v2;
        }
        (res1, res2)
    }
    fn duration(&self) -> i32 {
        let mut duration: i32 = 0;
        for source in self.sources.iter() {
            duration = duration.max((*source).duration());
        }
        duration
    }
}

}