pub mod cauchy_transfer {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

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

pub struct CauchyTransferState {
    source_state: SoundData
}

impl SoundSource for CauchyTransfer {
    fn init_state(&self) -> SoundData {
        Box::new(CauchyTransferState { source_state: self.source.init_state() })
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<CauchyTransferState>().unwrap();
        let val = self.source.next_value(n, &mut data.source_state);
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
