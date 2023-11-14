pub mod time_box {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

#[derive(Clone)]
pub struct TimeBox {
    duration: i32,
    ramp_time: i32,
    source: DynSoundSource,
}

impl TimeBox {
    pub fn new(
        duration: i32,
        ramp_time: i32,
        source: DynSoundSource
    ) -> Self {
        TimeBox{
            duration: duration,
            ramp_time: ramp_time,
            source: source
        }
    }
}

pub struct TimeBoxState {
    source_state: SoundData
}

impl SoundSource for TimeBox {
    fn init_state(&self) -> SoundData {
        Box::new(TimeBoxState { source_state: self.source.init_state() })
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<TimeBoxState>().unwrap();
        let source_val = self.source.next_value(n, &mut data.source_state);
        let mut gain = 1.0;
        if n < self.ramp_time {
            // ramp up start of note to avoid discontinuity
            gain *= 1.0 - (self.ramp_time - n) as f32 / self.ramp_time as f32;
        }
        if n > self.duration - self.ramp_time {
            // ramp down to end of note to avoid discontinuity
            gain *= 1.0 - (n - (self.duration - self.ramp_time)) as f32 / self.ramp_time as f32;
        }
        if gain < 0.0 || n > self.duration {
            gain = 0.0;
        }
        (source_val.0 * gain, source_val.1 * gain)
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let duration = params[0].parse::<f32>().unwrap() * reader.sample_rate as f32;
        let ramp_time = params[1].parse::<f32>().unwrap() * reader.sample_rate as f32;
        let source = reader.get_sound(&params[2]);
        Box::new(Self::new(duration.round() as i32, ramp_time.round() as i32, source))
    }
}

}
