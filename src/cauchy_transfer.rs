pub mod cauchy_transfer {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource};

pub struct CauchyTransfer {
    source: DynSoundSource,
}

impl CauchyTransfer {
    pub fn new(
        source: DynSoundSource,
        ) -> Self {
        CauchyTransfer{
            source: source,
        }
    }
}

fn transfer(x:f32) -> f32 {
    1.0 / ( 1.0 + x * x)
}

impl SoundSource for CauchyTransfer {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        let val = self.source.next_value(n);
        (transfer(val.0), transfer(val.1))
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
