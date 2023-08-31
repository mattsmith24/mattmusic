pub mod ramp {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

use crate::knob::knob::Knob;

pub struct Ramp {
    period: Knob,
    amplitude: Knob,
    duration: i32,
}

impl Ramp {
    pub fn new(
        period: Knob,
        amplitude: Knob,
        duration: i32,
        ) -> Self {
        Ramp{
            period: period,
            amplitude: amplitude,
            duration: duration,
        }
    }
}

struct RampData {
    period_data: SoundData,
    amplitude_data: SoundData,
    period_lock: i32,
    period_start: i32,
}

impl SoundSource for Ramp {
    fn init_state(&self) -> SoundData {
        Box::new(RampData {
            period_data: self.period.init_state(),
            amplitude_data: self.amplitude.init_state(),
            period_lock: 0,
            period_start: 0
        })
    }
    fn next_value(&self, n: i32, status: &mut SoundData) -> (f32, f32) {
        if n < 0 || n > self.duration {
            (0.0, 0.0)
        } else {
            let data = &mut status.downcast_mut::<RampData>().unwrap();
            // lock until end of period
            if n >= data.period_start + data.period_lock {
                data.period_lock = self.period.next_value(n, &mut data.period_data) as i32;
                data.period_start = n;
            }
            // generate ramp as 0.0 to 1.0
            let ramp = (n - data.period_start) as f32 / data.period_lock as f32;
            // multiply by amplitude
            let res = ramp * self.amplitude.next_value(n, &mut data.amplitude_data);
            (res, res)
        }
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let period = reader.get_knob(&params[0], reader.sample_rate as f32);
        let amplitude = reader.get_knob(&params[1], 1.0);
        let duration = params[2].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(Self::new(period, amplitude, duration.round() as i32))
    }
}

}
