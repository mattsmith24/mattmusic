pub mod experiment {

    //use std::path::Path;

    use crate::traits::traits::{DynSoundSource, Instrument} ;
    use crate::knob::knob::Knob;
    // use crate::square::square::Square;
    //use crate::triangle::triangle::Triangle;
    //use crate::pure_tone::pure_tone::PureTone;
    // use crate::dc::dc::DC;
    //use crate::lfo::lfo::LFO;
    // use crate::mix::mix::Mix;
    // use crate::envelope::envelope::{Envelope, EnvelopePoint};
    use crate::multiply::multiply::Multiply;
    // use crate::low_pass_filter::low_pass_filter::LowPassFilter;
    use crate::pre_render::pre_render::PreRender;
    //use crate::midi_notes::midi_notes::note2freq;
    //use crate::midi_notes::midi_notes as mn;
    use crate::time_box::time_box::TimeBox;
    // use crate::noise::noise::Noise;
    use crate::sine::sine::Sine;

    pub struct Experiment {
    }

    impl Experiment {
        pub fn new(_sample_rate: i32) -> Self {
            Experiment { }
        }
        fn patch(&self, freq: f32, duration: i32, strength: f32) -> DynSoundSource {
            let sine1 = Sine::new(Knob::dc(freq), Knob::dc(strength), duration);
            let sine2 = Sine::new(Knob::dc(freq*1.015), Knob::dc(strength), duration);
            let sine3 = Sine::new(Knob::dc(freq*0.503), Knob::dc(strength), duration);
            let sine4 = Sine::new(Knob::dc(freq*1.496), Knob::dc(strength*0.5), duration);
            let sine5 = Sine::new(Knob::dc(freq*2.01), Knob::dc(strength*0.25), duration);

            let mut mix = Multiply::new();
            mix.add(Box::new(sine1), strength);
            mix.add(Box::new(sine2), strength);
            mix.add(Box::new(sine3), strength);
            mix.add(Box::new(sine4), strength);
            mix.add(Box::new(sine5), strength);
            Box::new(mix)
        }
    }

    impl Instrument for Experiment {
        fn play(&self, freq: f32, duration: i32, strength: f32) -> DynSoundSource {
            let p1 = self.patch(freq, duration, strength);
            let p2 = self.patch(freq * 2.0, duration, strength);
            let mut mix = Multiply::new();
            //mix.add(Box::new(DC::new(1.0, duration)), 0.0);
            mix.add(p1, strength);
            mix.add(p2, strength);
            let timebox = TimeBox::new(duration, 88, Box::new(mix));
            let output = PreRender::new(Box::new(timebox));
            Box::new(output)
        }
    }

    }