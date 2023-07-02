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
    use crate::multiply::multiply::Multiply;
    
    
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
            // A long volume envelope that strengthens in the middle then trails off
            let mut points = Vec::<EnvelopePoint>::new();
            points.push(EnvelopePoint::new( 0.05,  strength ));
            points.push(EnvelopePoint::new( 0.1,  strength * 0.5 ));
            points.push(EnvelopePoint::new( 1.0,  strength ));
            points.push(EnvelopePoint::new( 3.85,  0.0 ));
            let envelope = Envelope::new(points);
            // An envelope to ensure the start and end of the notes aren't discontinuities
            // (avoids a pop sound at the start and end of notes)
            let mut points2 = Vec::<EnvelopePoint>::new();
            points2.push(EnvelopePoint::new( 0.001, 1.0 ));
            points2.push(EnvelopePoint::new( duration - 0.002, 1.0 ));
            points2.push(EnvelopePoint::new( 0.001, 0.0 ));
            let clip_off = Envelope::new(points2);
            // multiply the two envelopes to make them work together
            let mut multiplier = Multiply::new();
            multiplier.add(Box::new(envelope));
            multiplier.add(Box::new(clip_off));
            
            // Apparently brass sounds can be made by frequency modulation proportional to the amplitude
            let mut points = Vec::<EnvelopePoint>::new();
            points.push(EnvelopePoint::new( 0.05,  1.0 ));
            points.push(EnvelopePoint::new( 0.1,  0.5 ));
            points.push(EnvelopePoint::new( 1.0,  1.0 ));
            points.push(EnvelopePoint::new( 3.85,  0.0 ));
            let envelope = Envelope::new(points);
            let freq_knob = Knob::new(Box::new(LFO::new(freq, freq, 0.0, Knob::new(Box::new(envelope)), duration)));
            //let freq_knob = Knob::dc(freq);
            // strength is the gain for oscillators
            let strength_knob = Knob::new(Box::new(multiplier));
            Box::new(PureTone::new(
                //self.sample_rate,
                freq_knob,
                strength_knob,
                duration))
        }
    }
    
    }