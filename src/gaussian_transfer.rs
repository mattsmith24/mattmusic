pub mod gaussian_transfer {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource};

pub struct GaussianTransfer {
    source: DynSoundSource,
}

impl GaussianTransfer {
    pub fn new(
        source: DynSoundSource,
        ) -> Self {
        GaussianTransfer{
            source: source,
        }
    }
}

fn eminusxsq(x:f32) -> f32 {
    std::f32::consts::E.powf(-x * x)
}

impl SoundSource for GaussianTransfer {
    fn next_value(&self, n: i32) -> (f32, f32) {
        let val = self.source.next_value(n);
        (eminusxsq(val.0), eminusxsq(val.1))
    }

    fn duration(&self) -> i32 {
        self.source.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let source = reader.get_sound(&params[0]);
        Box::new(Self::new(source))
    }
}

}
