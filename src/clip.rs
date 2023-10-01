pub mod clip {

    use crate::read_song::read_song::SongReader;
    use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

    #[derive(Clone)]
    pub struct Clip
    {
        limit: f32,
        source: DynSoundSource,
    }

    impl Clip {
        pub fn new(limit: f32, source: DynSoundSource) -> Self {
            Clip { limit: limit, source: source }
        }
    }

    pub struct ClipState {
        source_state: SoundData
    }

    impl SoundSource for Clip {
        fn init_state(&self) -> SoundData {
            Box::new(ClipState { source_state: self.source.init_state() })
        }

        fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
            let data = &mut state.downcast_mut::<ClipState>().unwrap();
            let (mut v0, mut v1) = self.source.next_value(n, &mut data.source_state);
            v0 = v0.min(self.limit);
            v0 = v0.max(-self.limit);
            v1 = v1.min(self.limit);
            v1 = v1.max(-self.limit);
            (v0, v1)
        }

        fn duration(&self) -> i32 {
            self.source.duration()
        }

        fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
            Box::new(Clip::new(
                params[0].parse::<f32>().unwrap(),
                reader.get_sound(&params[1])
            ))
        }
    }
}