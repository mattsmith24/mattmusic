pub mod hann_window {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource};

pub struct HannWindow {
    source: DynSoundSource,
}

impl HannWindow {
    pub fn new(
        source: DynSoundSource,
        ) -> Self {
        HannWindow{
            source: source,
        }
    }
}

fn window(mut x:f32) -> f32 {
    // wrap x to lie between -pi and pi
    while x > std::f32::consts::PI {
        x -= 2.0 * std::f32::consts::PI;
    }
    while x < -std::f32::consts::PI {
        x += 2.0 * std::f32::consts::PI;
    }
    (x.cos() + 1.0) / 2.0
}

impl SoundSource for HannWindow {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        let val = self.source.next_value(n);
        (window(val.0), window(val.1))
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
