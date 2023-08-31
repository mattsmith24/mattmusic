pub mod cos_transfer {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

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

pub struct CosTransferState {
    source_state: SoundData
}

impl SoundSource for CosTransfer {
    fn init_state(&self) -> SoundData {
        Box::new(CosTransferState { source_state: self.source.init_state() })
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<CosTransferState>().unwrap();
        let mut val = self.source.next_value(n, &mut data.source_state);
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
