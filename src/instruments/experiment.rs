pub mod experiment {

    //use std::path::Path;

    use crate::traits::traits::{DynSoundSource, Instrument} ;
    use crate::knob::knob::Knob;
    use crate::square::square::Square;
    //use crate::triangle::triangle::Triangle;
    //use crate::pure_tone::pure_tone::PureTone;
    use crate::dc::dc::DC;
    //use crate::lfo::lfo::LFO;
    use crate::mix::mix::Mix;
    use crate::envelope::envelope::{Envelope, EnvelopePoint};
    use crate::multiply::multiply::Multiply;
    use crate::low_pass_filter::low_pass_filter::LowPassFilter;
    use crate::pre_render::pre_render::PreRender;
    //use crate::midi_notes::midi_notes::note2freq;
    //use crate::midi_notes::midi_notes as mn;
    use crate::time_box::time_box::TimeBox;
    use crate::noise::noise::Noise;
    use crate::sine::sine::Sine;

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
        fn play(&self, freq: f32, duration: i32, strength: f32) -> DynSoundSource {
            let decay = 0.100;
            let pitch_scale = 1.0;
            let filter_scale = 1.5;
            let sine_scale = 1.0;
            let mut points = Vec::<EnvelopePoint>::new();
            points.push(EnvelopePoint::new( self.t2n(0.000),  strength * 0.5 ));
            points.push(EnvelopePoint::new( self.t2n(0.005),  strength ));
            points.push(EnvelopePoint::new( self.t2n(0.060),  strength * 0.5 ));
            points.push(EnvelopePoint::new( self.t2n(decay),  0.0 ));
            let envelope = Envelope::new(points);

            let upper = freq / 2.5;
            let lower = freq / 5.0;
            let grad = upper - lower;
            let mut points = Vec::<EnvelopePoint>::new();
            points.push(EnvelopePoint::new( self.t2n(0.000),  1.0 * grad + lower ));
            points.push(EnvelopePoint::new( self.t2n(0.005),  0.5 * grad + lower ));
            points.push(EnvelopePoint::new( self.t2n(decay),  0.0 * grad + lower ));
            let pitch_envelope = Envelope::new(points);
            let filter_envelope = pitch_envelope.clone();
            let sine_envelope = pitch_envelope.clone();

            let mut pitch_scale_multiply = Multiply::new();
            pitch_scale_multiply.add(Box::new(pitch_envelope));
            pitch_scale_multiply.add(Box::new(DC::new(pitch_scale, duration)));
            let square = Square::new(
                Knob::new(Box::new(pitch_scale_multiply)),
                Knob::dc(1.0),
                duration);

            let noise = Noise::new(duration);

            let mut mix = Mix::new();
            mix.add(Box::new(square));
            mix.add(Box::new(noise));
            let mut filter_envelope_scale = Multiply::new();
            filter_envelope_scale.add(Box::new(filter_envelope));
            filter_envelope_scale.add(Box::new(DC::new(filter_scale, duration)));
            let filter = LowPassFilter::new(
                Knob::new(Box::new(filter_envelope_scale)),
                300,
                Box::new(mix));

            let mut sine_envelope_multiply = Multiply::new();
            sine_envelope_multiply.add(Box::new(sine_envelope));
            sine_envelope_multiply.add(Box::new(DC::new(sine_scale, duration)));
            let sine = Sine::new(
                Knob::new(Box::new(sine_envelope_multiply)),
                Knob::dc(1.0),
                duration);

            let mut multiply = Multiply::new();
            multiply.add(Box::new(filter));
            multiply.add(Box::new(envelope));
            multiply.add(Box::new(sine));

            let timebox = TimeBox::new(duration, Box::new(multiply));

            let output = PreRender::new(Box::new(timebox));
            // if !Path::new("multiply.csv").exists() {
            //     let _ = multiply.debug("multiply.csv");
            // }
            Box::new(output)
        }
    }

    }