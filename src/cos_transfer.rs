pub mod cos_transfer {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource};

pub struct CosTransfer {
    source: DynSoundSource,
}

impl CosTransfer {
    pub fn new(
        source: DynSoundSource,
        ) -> Self {
        CosTransfer{
            source: source,
        }
    }
}

impl SoundSource for CosTransfer {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        let mut val = self.source.next_value(n);
        val.0 = (val.0 * 2.0 * std::f32::consts::PI).cos();
        val.1 = (val.1 * 2.0 * std::f32::consts::PI).cos();
        val
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
