pub mod delay_line {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

use crate::knob::knob::Knob;

const MAX_DELAY: i32 = 48000 * 10;

pub struct DelayLine {
    input: DynSoundSource,
    delay: Knob,
}

impl DelayLine {
    pub fn new(input: DynSoundSource, delay: Knob) -> Self {
        DelayLine { input: input, delay: delay}
    }
}

struct DelayLineData {
    input_datam1: SoundData,
    input_data0: SoundData,
    input_data1: SoundData,
    input_data2: SoundData,
    delay_data: SoundData,
}

impl SoundSource for DelayLine {
    fn init_state(&self) -> SoundData {
        Box::new(DelayLineData {
            // Keep four states for each of the delays we use to calculate the
            // cubic interpolation.
            input_datam1: self.input.init_state(),
            input_data0: self.input.init_state(),
            input_data1: self.input.init_state(),
            input_data2: self.input.init_state(),
            delay_data: self.delay.init_state(),
        })
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<DelayLineData>().unwrap();
        let d = self.delay.next_value(n, &mut data.delay_data);
        let x0 = (n as f32 - d).floor() as i32 - 2;
        if x0 >= 1 && d >= 0.0 && d.round() as i32 <= MAX_DELAY {
            // Cubic interpolation
            // We apply a base delay of 2 so we don't have to see into the future
            // when getting x0 + 2
            let ym1 = self.input.next_value(x0 - 1, &mut data.input_datam1); // y[x0 - 1]
            let y0 = self.input.next_value(x0, &mut data.input_data0); // y[x0]
            let y1 = self.input.next_value(x0 + 1, &mut data.input_data1); // y[x0 + 1]
            let y2 = self.input.next_value(x0 + 2, &mut data.input_data2); // y[x0 + 2]
            let f = (n as f32 - d) - x0 as f32;
            let output0 = -f * (f - 1.0) * (f - 2.0) / 6.0 * ym1.0
                + (f + 1.0) * (f - 1.0) * (f - 2.0) / 2.0 * y0.0
                -(f + 1.0) * f * (f - 2.0) / 2.0 * y1.0
                + (f + 1.0) * f * (f - 1.0) / 6.0 * y2.0;
            let output1 = -f * (f - 1.0) * (f - 2.0) / 6.0 * ym1.1
                + (f + 1.0) * (f - 1.0) * (f - 2.0) / 2.0 * y0.1
                -(f + 1.0) * f * (f - 2.0) / 2.0 * y1.1
                + (f + 1.0) * f * (f - 1.0) / 6.0 * y2.1;
            (output0, output1)
        } else {
            (0.0, 0.0)
        }
    }

    fn duration(&self) -> i32 {
        self.input.duration() + MAX_DELAY
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let delay = reader.get_knob(&params[1], 1.0);
        Box::new(DelayLine::new(input, delay))
    }
}
}