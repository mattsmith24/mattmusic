pub mod experiment {

    use std::path::Path;

    use crate::traits::traits::{DynSoundSource, Instrument} ;
    use crate::knob::knob::Knob;
    use crate::square::square::Square;
    //use crate::triangle::triangle::Triangle;
    //use crate::pure_tone::pure_tone::PureTone;
    use crate::dc::dc::DC;
    //use crate::lfo::lfo::LFO;
    //use crate::mix::mix::Mix;
    use crate::envelope::envelope::{Envelope, EnvelopePoint};
    use crate::multiply::multiply::Multiply;
    //use crate::low_pass_filter::low_pass_filter::LowPassFilter;
    use crate::pre_render::pre_render::PreRender;
    use crate::midi_notes::midi_notes::note2freq;
    use crate::midi_notes::midi_notes as mn;



    pub struct Experiment {
        sample_rate: i32,
    }

    impl Experiment {
        pub fn new(sample_rate: i32) -> Self {
            Experiment { sample_rate: sample_rate }
        }
        fn t2n(&self, t: f32) -> i32 {
            (t * self.sample_rate as f32).round() as i32
        }
    }

    impl Instrument for Experiment {
        fn play(&self, _freq: f32, duration: i32, strength: f32) -> DynSoundSource {
            let decay = 0.500;
            let mut points = Vec::<EnvelopePoint>::new();
            points.push(EnvelopePoint::new( self.t2n(0.000),  strength * 0.5 ));
            points.push(EnvelopePoint::new( self.t2n(0.005),  strength ));
            points.push(EnvelopePoint::new( self.t2n(0.060),  strength * 0.5 ));
            points.push(EnvelopePoint::new( self.t2n(decay),  0.0 ));
            let envelope = Envelope::new(points);

            let upper = note2freq(5, mn::MIDI_OFFSET_G_SHARP) / self.sample_rate as f32;
            let lower = note2freq(1, mn::MIDI_OFFSET_F) / self.sample_rate as f32;
            let grad = upper - lower;
            let mut points = Vec::<EnvelopePoint>::new();
            points.push(EnvelopePoint::new( self.t2n(0.000),  1.0 * grad + lower ));
            points.push(EnvelopePoint::new( self.t2n(0.005),  0.5 * grad + lower ));
            points.push(EnvelopePoint::new( self.t2n(decay),  0.0 * grad + lower ));
            let pitch_envelope = Envelope::new(points);

            let mut pitch_scale = Multiply::new();
            pitch_scale.add(Box::new(pitch_envelope));
            pitch_scale.add(Box::new(DC::new(0.5, duration)));
            let square = Square::new(
                Knob::new(Box::new(pitch_scale)),
                Knob::dc(1.0),
                duration);
            let mut multiply = Multiply::new();
            multiply.add(Box::new(square));
            multiply.add(Box::new(envelope));
            // let multiply = PreRender::new(Box::new(multiply));
            // if !Path::new("multiply.csv").exists() {
            //     let _ = multiply.debug("multiply.csv");
            // }
            Box::new(multiply)
        }
    }

    }