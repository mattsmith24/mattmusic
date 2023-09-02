pub mod gaussian_transfer {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

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

struct GaussianTransferData {
    source_data: SoundData,
}

impl SoundSource for GaussianTransfer {
    fn init_state(&self) -> SoundData {
        Box::new(GaussianTransferData { source_data: self.source.init_state() })
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<GaussianTransferData>().unwrap();
        let val = self.source.next_value(n, &mut data.source_data);
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
