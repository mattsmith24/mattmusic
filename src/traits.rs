pub mod traits {

use std::any::Any;
use dyn_clone::DynClone;
use num::complex::Complex;

use crate::read_song::read_song::SongReader;

pub type SoundData = Box<dyn Any + Send + Sync>;

pub trait SoundSource: DynClone {
    fn init_state(&self) -> SoundData;
    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32);
    fn duration(&self) -> i32;
    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource where Self: Sized;
}
dyn_clone::clone_trait_object!(SoundSource);
pub type DynSoundSource = Box<dyn SoundSource + Send + Sync>;

pub trait ComplexSoundSource: DynClone {
    fn init_state(&self) -> SoundData;
    fn next_value(&self, n: i32, state: &mut SoundData) -> (Complex::<f32>, Complex::<f32>);
    fn duration(&self) -> i32;
}
dyn_clone::clone_trait_object!(ComplexSoundSource);
pub type DynComplexSoundSource = Box<dyn ComplexSoundSource + Send + Sync>;

pub trait Instrument {
    fn play(&self, freq: f32, duration: i32, strength: f32) -> DynSoundSource;
}
pub type DynInstrument = Box<dyn Instrument + Send + Sync>;

}
