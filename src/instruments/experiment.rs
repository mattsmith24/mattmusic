pub mod experiment {

    use crate::traits::traits::{DynSoundSource, Instrument} ;
    use crate::knob::knob::Knob;
    use crate::square::square::Square;
    use crate::triangle::triangle::Triangle;
    use crate::pure_tone::pure_tone::PureTone;
    use crate::dc::dc::DC;
    use crate::lfo::lfo::LFO;
    use crate::mix::mix::Mix;
    
    
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
            Box::new(PureTone::new(
                //self.sample_rate,
                Knob::new(Box::new(LFO::new(freq, 1.01, 10.0, duration))),
                Knob::dc(strength),
                duration))
        }
    }
    
    }