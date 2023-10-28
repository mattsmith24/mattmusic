pub mod mix {

    use crate::read_song::read_song::SongReader;
    use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

    use crate::dc::dc::DC;
    use crate::sequence::sequence::Sequence;

    #[derive(Clone)]
    pub struct Mix
    {
        sequence: Sequence,
    }

    impl Mix {
        pub fn new() -> Self {
            Mix { sequence: Sequence::new() }
        }
        pub fn add(&mut self, source: DynSoundSource) -> &mut Mix {
            self.sequence.add(0, source);
            self
        }
    }

    struct MixData {
        sequence_data: SoundData
    }

    impl SoundSource for Mix {
        fn init_state(&self) -> SoundData {
            Box::new(MixData{sequence_data: self.sequence.init_state()})
        }

        fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
            let data = state.downcast_mut::<MixData>().unwrap();
            self.sequence.next_value(n, &mut data.sequence_data)
        }

        fn duration(&self) -> i32 {
            self.sequence.duration()
        }

        fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
            let mut mix = Mix::new();
            let mut max_duration = 0.0;
            for param in params {
                println!("Mix::from_yaml(param: {})", param);
                let parts: Vec<_> = param.split(" ").collect();
                // If the first token is 'dc' then we expect the following to be the value and duration
                // Otherwise we expect to see a dc offset and a source name
                if parts[0] == "dc" {
                    let val = parts[1].parse::<f32>().unwrap();
                    let duration: f32;
                    // If the dc component duration token is "max" then we use the
                    // running maximum duration of any previous sources.
                    if parts[2] == "max" {
                        duration = max_duration;
                        println!("max: duration = {}", duration);
                    } else {
                        duration = parts[2].parse::<f32>().unwrap() * reader.sample_rate as f32;
                    }
                    let source = Box::new(DC::new(val, duration.round() as i32));
                    mix.add(source);
                } else {
                    let source = reader.get_sound(param);
                    max_duration = max_duration.max(source.duration() as f32);
                    mix.add(source);
                }
            }
            Box::new(mix)
        }
    }
}