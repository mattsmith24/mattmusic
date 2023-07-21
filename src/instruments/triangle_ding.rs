pub mod triangle_ding {

use crate::traits::traits::{DynSoundSource, Instrument} ;
use crate::knob::knob::Knob;
use crate::triangle::triangle::Triangle;
use crate::envelope::envelope::{Envelope, EnvelopePoint};
use crate::sine::sine::Sine;
use crate::dc::dc::DC;
use crate::time_box::time_box::TimeBox;
use crate::multiply::multiply::Multiply;


pub struct TriangleDing {
    sample_rate: i32,
}

impl TriangleDing {
    pub fn new(sample_rate: i32) -> Self {
        TriangleDing { sample_rate: sample_rate }
    }
    fn t2n(&self, t: f32) -> i32 {
        (t * self.sample_rate as f32).round() as i32
    }
}

impl Instrument for TriangleDing {
    fn play(&self, freq: f32, duration: i32, strength: f32) -> DynSoundSource {
        let mut points = Vec::<EnvelopePoint>::new();
        points.push(EnvelopePoint::new( self.t2n(0.005),  1.0 ));
        points.push(EnvelopePoint::new( self.t2n(0.1),  0.5 ));
        points.push(EnvelopePoint::new( self.t2n(2.0),  0.0 ));
        let envelope = Envelope::new(points);

        let tremolo_wave = Sine::new(Knob::dc(5.0 / self.sample_rate as f32), Knob::dc(0.25), duration);

        let sine = Triangle::new(Knob::dc(freq), Knob::dc(1.0), duration);

        let mut multiply = Multiply::new();
        multiply.add(Box::new(sine), 1.0);
        multiply.add(Box::new(envelope), 0.0);
        multiply.add(Box::new(DC::new(strength, duration)), 0.0);
        multiply.add(Box::new(tremolo_wave), 1.0);

        Box::new(TimeBox::new(duration, Box::new(multiply)))
    }
}

}