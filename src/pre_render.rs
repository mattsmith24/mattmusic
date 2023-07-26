pub mod pre_render {

// use std::fs::File;
// use std::io::{Result, Write};
use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource};


pub struct PreRender {
    rendered_sound_source: Vec::<(f32,f32)>
}

impl PreRender {
    pub fn new(mut source: DynSoundSource) -> Self {
        let mut buf = Vec::<(f32, f32)>::new();
        let mut sample_clock = 0i32;
        let duration = (*source).duration();
        while sample_clock < duration {
            buf.push((*source).next_value(sample_clock));
            sample_clock += 1;
        }
        PreRender {
            rendered_sound_source: buf
        }
    }

    // pub fn debug(&self, path: &str) -> Result<()> {
    //     let mut output = File::create(path)?;
    //     for n in 0..self.rendered_sound_source.len() {
    //         let s = self.rendered_sound_source[n];
    //         write!(output, "{}, {}\n", s.0, s.1)?;
    //     }
    //     Ok(())
    // }
}

impl SoundSource for PreRender {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        if n < self.rendered_sound_source.len() as i32 {
            self.rendered_sound_source[n as usize]
        } else {
            (0.0, 0.0)
        }
    }

    fn duration(&self) -> i32 {
        self.rendered_sound_source.len() as i32
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let source = reader.get_sound(&params[0]);
        Box::new(Self::new(source))
    }
}

}
