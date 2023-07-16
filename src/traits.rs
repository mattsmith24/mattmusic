pub mod traits {
use crate::read_song::read_song::YAMLFormat;
pub trait SoundSource {
    fn next_value(&mut self, n: i32) -> (f32, f32);
    fn duration(&self) -> i32;
    fn from_yaml(params: &Vec::<String>, yaml: &YAMLFormat, sample_rate: i32) -> DynSoundSource where Self: Sized;
}
pub type DynSoundSource = Box<dyn SoundSource + Send + Sync>;

pub trait Instrument {
    fn play(&self, freq: f32, duration: i32, strength: f32) -> DynSoundSource;
}
pub type DynInstrument = Box<dyn Instrument + Send + Sync>;
}