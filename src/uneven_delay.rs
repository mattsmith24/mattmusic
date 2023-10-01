pub mod uneven_delay {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

#[derive(Clone)]
pub struct UnevenDelay {
    input: DynSoundSource,
    left_delay: i32,
    right_delay: i32,
}

impl UnevenDelay {
    pub fn new(input: DynSoundSource, left_delay: i32, right_delay: i32) -> Self {
        UnevenDelay { input: input, left_delay: left_delay, right_delay: right_delay }
    }
}

struct UnevenDelayData {
    left_input_data: SoundData,
    right_input_data: SoundData,
}

impl SoundSource for UnevenDelay {
    fn init_state(&self) -> SoundData {
        // The input is initialised once for each channel so that we can
        // confidently interleave differently delayed versions of the input and
        // not worry about their states interfering
        Box::new( UnevenDelayData {
            left_input_data: self.input.init_state(),
            right_input_data: self.input.init_state(),
         } )
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<UnevenDelayData>().unwrap();
        let left;
        if n >= self.left_delay {
            left = self.input.next_value(n - self.left_delay, &mut data.left_input_data);
        } else {
            left = (0.0, 0.0);
        }
        let right;
        if n >= self.right_delay {
            right = self.input.next_value(n - self.right_delay, &mut data.right_input_data);
        } else {
            right = (0.0, 0.0);
        }
        (left.0, right.1)
    }

    fn duration(&self) -> i32 {
        self.input.duration() + (self.left_delay).max(self.right_delay)
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let left_delay = params[1].parse::<f32>().unwrap() * reader.sample_rate as f32;
        let right_delay = params[2].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(UnevenDelay::new(input, left_delay.round() as i32, right_delay.round() as i32))
    }
}

}
