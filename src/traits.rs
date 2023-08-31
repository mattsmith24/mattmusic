pub mod traits {

use std::any::Any;
use crate::read_song::read_song::SongReader;

pub type SoundData = Box<dyn Any + Send + Sync>;

pub trait SoundSource {
    fn init_state(&self) -> SoundData;
    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32);
    fn duration(&self) -> i32;
    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource where Self: Sized;
}
pub type DynSoundSource = Box<dyn SoundSource + Send + Sync>;

pub trait Instrument {
    fn play(&self, freq: f32, duration: i32, strength: f32) -> DynSoundSource;
}
pub type DynInstrument = Box<dyn Instrument + Send + Sync>;

}
