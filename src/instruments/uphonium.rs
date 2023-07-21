pub mod uphonium {

    use crate::traits::traits::{DynSoundSource, Instrument} ;
    use crate::knob::knob::Knob;
    use crate::sine::sine::Sine;
    use crate::envelope::envelope::{Envelope, EnvelopePoint};
    use crate::multiply::multiply::Multiply;
    use crate::low_pass_filter::low_pass_filter::LowPassFilter;
    use crate::pre_render::pre_render::PreRender;
    use crate::generative_waveform::generative_waveform::GenerativeWaveform;


    pub struct Uphonium {
        sample_rate: i32,
    }

    impl Uphonium {
        pub fn new(sample_rate: i32) -> Self {
            Uphonium { sample_rate: sample_rate }
        }

        fn t2n(&self, t: f32) -> i32 {
            (t * self.sample_rate as f32).round() as i32
        }
    }

    impl Instrument for Uphonium {
        fn play(&self, freq: f32, duration: i32, strength: f32) -> DynSoundSource {
            // A long volume envelope that strengthens in the middle then trails off
            let mut points = Vec::<EnvelopePoint>::new();
            points.push(EnvelopePoint::new( self.t2n(0.05),  strength ));
            points.push(EnvelopePoint::new( self.t2n(0.1),  strength * 0.5 ));
            points.push(EnvelopePoint::new( self.t2n(1.0),  strength ));
            points.push(EnvelopePoint::new( self.t2n(3.85),  0.0 ));
            let envelope = Envelope::new(points);
            // An envelope to ensure the start and end of the notes aren't discontinuities
            // (avoids a pop sound at the start and end of notes)
            let mut points2 = Vec::<EnvelopePoint>::new();
            points2.push(EnvelopePoint::new( self.t2n(0.001), 1.0 ));
            points2.push(EnvelopePoint::new( duration - self.t2n(0.002), 1.0 ));
            points2.push(EnvelopePoint::new( self.t2n(0.001), 0.0 ));
            let clip_off = Envelope::new(points2);
            // multiply the two envelopes to make them work together
            let mut multiplier = Multiply::new();
            multiplier.add(Box::new(envelope), 0.0);
            multiplier.add(Box::new(clip_off), 0.0);

            // Apparently brass sounds can be made by frequency modulation proportional to the amplitude
            let pitch_envelope_gain = 1.0 / self.sample_rate as f32;
            let mut points = Vec::<EnvelopePoint>::new();
            points.push(EnvelopePoint::new( self.t2n(0.05),  1.0 * pitch_envelope_gain ));
            points.push(EnvelopePoint::new( self.t2n(0.1),  0.5 * pitch_envelope_gain ));
            points.push(EnvelopePoint::new( self.t2n(1.0),  1.0 * pitch_envelope_gain ));
            points.push(EnvelopePoint::new( self.t2n(3.85),  0.0 ));
            let envelope = Envelope::new(points);
            //
            let mut freq_multiplier = Multiply::new();
            freq_multiplier.add(Box::new(Sine::new(Knob::dc(freq), Knob::new(Box::new(envelope)), duration)), freq);
            let freq_knob = Knob::new(Box::new(freq_multiplier));
            // strength is the gain for oscillators
            let strength_knob = Knob::new(Box::new(multiplier));
            let pure_tone =  GenerativeWaveform::new(
                freq_knob,
                self.sample_rate * 2,
                1,
                strength_knob,
                false,
                duration);
            let low_pass = LowPassFilter::new(
                Knob::dc(2000.0/self.sample_rate as f32),
                100,
                Box::new(pure_tone)
            );
            Box::new(PreRender::new(Box::new(low_pass)))
        }
    }

    }