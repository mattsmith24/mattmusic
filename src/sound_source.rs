pub mod sound_source {
pub trait SoundSource {
    fn next_value(&self, t: f32) -> (f32, f32);
    fn duration(&self) -> f32;
}
pub type DynSoundSource = Box<dyn SoundSource + Send + Sync>;

pub trait Instrument {
    fn play(&self, freq: f32, duration: f32, strength: f32) -> DynSoundSource;
}
pub type DynInstrument = Box<dyn Instrument + Send + Sync>;
}