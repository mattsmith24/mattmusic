pub mod envelope {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};


#[derive(Clone)]
pub struct EnvelopePoint {
    time_offset: i32, // each time offset is relative to the previous point
    value: f32
}

impl EnvelopePoint {
    pub fn new(time_offset: i32, value: f32) -> Self {
        EnvelopePoint { time_offset: time_offset, value: value }
    }
}

#[derive(Clone)]
pub struct Envelope {
    points: Vec::<EnvelopePoint>
}

impl Envelope {
    pub fn new(points: Vec::<EnvelopePoint>) -> Self {
        Envelope { points: points }
    }
}
impl SoundSource for Envelope {
    fn init_state(&self) -> SoundData {
        Box::new(0)
    }

    fn next_value(&self, n: i32, _state: &mut SoundData) -> (f32, f32) {
        let mut point_start_time = 0;
        let mut output = 0.0;
        for point in &self.points {
            if n == 0 && point.time_offset == 0 {
                output = point.value;
                break;
            } else if n < point_start_time + point.time_offset {
                output = (n - point_start_time) as f32 / point.time_offset as f32 * (point.value - output) + output;
                break;
            } else {
                point_start_time += point.time_offset;
                output = point.value;
            }
        }
        (output, output)
    }

    fn duration(&self) -> i32 {
        let mut res = 0;
        for point in &self.points {
            res += point.time_offset;
        }
        res
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let mut points = Vec::<EnvelopePoint>::new();
        for param in params {
            println!("Envelope::from_yaml param {}", &param);
            let parts: Vec<_> = param.split(" ").collect();
            let time_offset = parts[0].parse::<f32>().unwrap() * reader.sample_rate as f32;
            let value = parts[1].parse::<f32>().unwrap();
            points.push(EnvelopePoint::new(time_offset.round() as i32, value));
        }
        Box::new(Self::new(points))
    }
}

}