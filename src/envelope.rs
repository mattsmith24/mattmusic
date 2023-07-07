pub mod envelope {

use crate::traits::traits::SoundSource;

pub struct EnvelopePoint {
    time_offset: f32, // each time offset is relative to the previous point
    value: f32
}

impl EnvelopePoint {
    pub fn new(time_offset: f32, value: f32) -> Self {
        EnvelopePoint { time_offset: time_offset, value: value }
    }
}

pub struct Envelope {
    points: Vec::<EnvelopePoint>
}

impl Envelope {
    pub fn new(points: Vec::<EnvelopePoint>) -> Self {
        Envelope { points: points }
    }
}
impl SoundSource for Envelope {
    fn next_value(&mut self, t: f32) -> (f32, f32) {
        let mut point_start_time = 0.0;
        let mut output = 0.0;
        for point in &self.points {
            if t < point_start_time + point.time_offset {
                output = (t - point_start_time) / point.time_offset * (point.value - output) + output;
                break;
            } else {
                point_start_time += point.time_offset;
                output = point.value;
            }
        }
        (output, output)
    }

    fn duration(&self) -> f32 {
        let mut res = 0.0;
        for point in &self.points {
            res += point.time_offset;
        }
        res
    }
}

}