pub mod mix {

    use crate::read_song::read_song::SongReader;
    use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

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
            for param in params {
                mix.add(reader.get_sound(param));
            }
            Box::new(mix)
        }
    }
}