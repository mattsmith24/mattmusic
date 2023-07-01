pub mod experiment {

    use crate::traits::traits::{DynSoundSource, Instrument} ;
    use crate::knob::knob::Knob;
    //use crate::square::square::Square;
    //use crate::triangle::triangle::Triangle;
    use crate::pure_tone::pure_tone::PureTone;
    //use crate::dc::dc::DC;
    use crate::lfo::lfo::LFO;
    //use crate::mix::mix::Mix;
    use crate::envelope::envelope::{Envelope, EnvelopePoint};
    
    
    pub struct Experiment {
        sample_rate: f32,
    }
    
    impl Experiment {
        pub fn new(sample_rate: f32) -> Self {
            Experiment { sample_rate: sample_rate }
        }
    }
    
    impl Instrument for Experiment {
        fn play(&self, freq: f32, duration: f32, strength: f32) -> DynSoundSource {
            let mut points = Vec::<EnvelopePoint>::new();
            points.push(EnvelopePoint::new( 0.05,  1.0 ));
            points.push(EnvelopePoint::new( 0.1,  0.5 ));
            points.push(EnvelopePoint::new( 1.0,  1.0 ));
            points.push(EnvelopePoint::new( 3.85,  0.0 ));
            let envelope = Envelope::new(points);
            //let freq_knob = Knob::new(Box::new(LFO::new(freq, 0.2, 0.25, 10.0, duration)));
            let freq_knob = Knob::dc(freq);
            let strength_knob = Knob::new(Box::new(envelope));
            Box::new(PureTone::new(
                //self.sample_rate,
                freq_knob,
                strength_knob,
                duration))
        }
    }
    
    }