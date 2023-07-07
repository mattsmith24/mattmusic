pub mod pre_render {

use std::fs::File;
use std::io::{Result, Error, Write};
use crate::traits::traits::{SoundSource, DynSoundSource};

pub struct PreRender {
    sample_rate: f32,
    rendered_sound_source: Vec::<(f32,f32)>
}

impl PreRender {
    pub fn new(sample_rate: f32, mut source: DynSoundSource) -> Self {
        let mut buf = Vec::<(f32, f32)>::new();
        let mut sample_clock = 0f32;
        let duration = (*source).duration();
        while sample_clock / sample_rate <= duration {
            buf.push((*source).next_value(sample_clock / sample_rate));
            sample_clock += 1.0;
        }
        PreRender {
            sample_rate: sample_rate,
            rendered_sound_source: buf
        }
    }

    pub fn debug(&self, path: &str) -> Result<()> {
        let mut output = File::create(path)?;
        for n in 0..self.rendered_sound_source.len() {
            let s = self.rendered_sound_source[n];
            write!(output, "{}, {}\n", s.0, s.1);
        }
        Ok(())
    }
}

impl SoundSource for PreRender {
    fn next_value(&mut self, t: f32) -> (f32, f32) {
        let n = (t * self.sample_rate).round() as usize;
        if n < self.rendered_sound_source.len() {
            self.rendered_sound_source[n]
        } else {
            (0.0, 0.0)
        }
    }

    fn duration(&self) -> f32 {
        self.rendered_sound_source.len() as f32 / self.sample_rate
    }
}

}
