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
    
    pub fn from_vec(sources: Vec::<DynSoundSource>) -> Self {
        Multiply{ sources: sources }
    }

}

impl SoundSource for Multiply {
    fn next_value(&self, t: f32) -> (f32, f32) {
        let mut res1: f32 = 1.0;
        let mut res2: f32 = 1.0;
        for source in self.sources.iter() {
            let (v1, v2) = (*source).next_value(t);
            res1 *= v1;
            res2 *= v2;
        }
        (res1, res2)
    }
    fn duration(&self) -> f32 {
        let mut duration: f32 = 0.0;
        for source in self.sources.iter() {
            duration += (*source).duration();
        }
        duration
    }
}

}