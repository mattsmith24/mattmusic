pub mod cos_transfer {

use std::sync::{Arc, Mutex};

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundState, DynSoundState};

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
    source_state: DynSoundState
}

const SOURCE_STATE: usize = 0;

impl SoundState for CosTransferState {
    fn get_sound_state(&self, key: usize) -> DynSoundState {
        match key {
            SOURCE_STATE => self.source_state,
            _ => panic!("CosTransferState unknown key {}", key)
        }
    }
}

impl SoundSource for CosTransfer {
    fn init_state(&self) -> DynSoundState {
        Arc::new(Mutex::new(CosTransferState { source_state: self.source.init_state() }))
    }

    fn next_value(&self, n: i32, state: DynSoundState) -> (f32, f32) {
        let data = state.lock().unwrap();
        let mut val = self.source.next_value(n, data.get_sound_state(SOURCE_STATE));
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
