pub mod clip {

    use crate::read_song::read_song::SongReader;
    use crate::traits::traits::{SoundSource, DynSoundSource};

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

    impl SoundSource for Clip {
        fn next_value(&mut self, n: i32) -> (f32, f32) {
            let (mut v0, mut v1) = self.source.next_value(n);
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