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
    fn next_value(&self, t: f32) -> (f32, f32) {
        let mut duration_sum = 0.0;
        let mut prev_value = 0.0;
        let mut output = 0.0;
        for point in &self.points {
            if t < duration_sum + point.time_offset {
                output = (t - duration_sum) / point.time_offset * (point.value - prev_value) + prev_value;
                break;
            } else {
                duration_sum += point.time_offset;
                prev_value = point.value;
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