pub mod traits {
pub trait SoundSource {
    fn next_value(&mut self, n: i32) -> (f32, f32);
    fn duration(&self) -> i32;
}
pub type DynSoundSource = Box<dyn SoundSource + Send + Sync>;

pub trait Instrument {
    fn play(&self, freq: f32, duration: i32, strength: f32) -> DynSoundSource;
}
pub type DynInstrument = Box<dyn Instrument + Send + Sync>;
}