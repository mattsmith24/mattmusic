pub mod clip {

    use std::sync::{Arc, Mutex};

    use crate::read_song::read_song::SongReader;
    use crate::traits::traits::{SoundSource, DynSoundSource, SoundState, DynSoundState};

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
        source_state: DynSoundState
    }

    const SOURCE_STATE: usize = 0;

    impl SoundState for ClipState {
        fn get_sound_state(&self, key: usize) -> DynSoundState {
            match key {
                SOURCE_STATE => self.source_state,
                _ => panic!("ClipState unknown key {}", key)
            }
        }
    }

    impl SoundSource for Clip {
        fn init_state(&self) -> DynSoundState {
            Arc::new(Mutex::new(ClipState { source_state: self.source.init_state() }))
        }

        fn next_value(&self, n: i32, state: DynSoundState) -> (f32, f32) {
            let data = state.lock().unwrap();
            let (mut v0, mut v1) = self.source.next_value(n, data.get_sound_state(SOURCE_STATE));
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