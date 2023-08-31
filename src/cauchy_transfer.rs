pub mod cauchy_transfer {

use std::sync::{Arc, Mutex};

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundState, DynSoundState};

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
    source_state: DynSoundState
}

const SOURCE_STATE: usize = 0;

impl SoundState for CauchyTransferState {
    fn get_sound_state(&self, key: usize) -> DynSoundState {
        match key {
            SOURCE_STATE => self.source_state,
            _ => panic!("CauchyTransferState unknown key {}", key)
        }
    }
}

impl SoundSource for CauchyTransfer {
    fn init_state(&self) -> DynSoundState {
        Arc::new(Mutex::new(CauchyTransferState { source_state: self.source.init_state() }))
    }

    fn next_value(&self, n: i32, state: DynSoundState) -> (f32, f32) {
        let data = state.lock().unwrap();
        let val = self.source.next_value(n, data.get_sound_state(SOURCE_STATE));
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
