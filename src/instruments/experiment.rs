pub mod experiment {

    use crate::traits::traits::{DynSoundSource, Instrument} ;
    use crate::knob::knob::Knob;
    use crate::square::square::Square;
    use crate::triangle::triangle::Triangle;
    use crate::pure_tone::pure_tone::PureTone;
    use crate::dc::dc::DC;
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
            // modulate the frequency knob
            let mut mix = Box::new(Mix::new());
            let dc = Box::new(DC::new(freq, duration));
            let wave = Box::new(PureTone::new(2.56677, 5.0, duration));
            mix.add(dc);
            mix.add(wave);
            let mut freq_knob = Knob::new(mix);
            freq_knob.set_debug(true);
            let mut strength_knob = Knob::new_dc(strength);
            //strength_knob.set_debug(true);
            Box::new(Triangle::new(
                self.sample_rate,
                freq_knob,
                strength_knob,
                duration))
        }
    }
    
    }