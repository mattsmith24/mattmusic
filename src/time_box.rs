pub mod time_box {

use crate::read_song::read_song::YAMLFormat;
use crate::traits::traits::{SoundSource, DynSoundSource};


pub struct TimeBox {
    duration: i32,
    source: DynSoundSource,
}

impl TimeBox {
    pub fn new(
        duration: i32,
        source: DynSoundSource
    ) -> Self {
        TimeBox{
            duration: duration,
            source: source
        }
    }
}

impl SoundSource for TimeBox {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        let source_val = (*self.source).next_value(n);
        let mut gain = 1.0;
        const LIFT: i32 = 44;
        if n > self.duration - LIFT {
            // ramp down to end of note to avoid discontinuity
            gain *= 1.0 - (n - (self.duration - LIFT)) as f32 / LIFT as f32;
        }
        if gain < 0.0 || n > self.duration {
            gain = 0.0;
        }
        (source_val.0 * gain, source_val.1 * gain)
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(params: &Vec::<String>, yaml: &YAMLFormat, sample_rate: i32) -> DynSoundSource {
        use crate::dc::dc::DC;
        todo!();
        Box::new(Self::new(0, Box::new(DC::new(0.0, 0))))
    }
}

}
