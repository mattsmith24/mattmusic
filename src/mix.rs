pub mod mix {

    use crate::read_song::read_song::SongReader;
    use crate::traits::traits::{SoundSource, DynSoundSource};

    use crate::sequence::sequence::Sequence;

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

    impl SoundSource for Mix {
        fn next_value(&mut self, n: i32) -> (f32, f32) {
            self.sequence.next_value(n)
        }

        fn duration(&self) -> i32 {
            self.sequence.duration()
        }

        fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
            use crate::dc::dc::DC;
            todo!();
            Box::new(Self::new())
        }
    }
}