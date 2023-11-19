pub mod pre_render {

// use std::fs::File;
// use std::io::{Result, Write};
use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

#[derive(Clone)]
pub struct PreRender {
    rendered_sound_source: Vec::<(f32,f32)>
}

impl PreRender {
    pub fn new(source: DynSoundSource) -> Self {
        let mut buf = Vec::<(f32, f32)>::new();
        let mut sample_clock = 0i32;
        let duration = source.duration();
        println!("PreRender {} samples", duration);
        let mut source_data = source.init_state();
        let mut report_threshold = 10.0;
        while sample_clock < duration {
            let percent_done = sample_clock as f32 * 100.0 / duration as f32;
            if percent_done > report_threshold {
                println!("Processed {} samples, {}%", sample_clock, percent_done);
                report_threshold += 10.0;
            }
            buf.push(source.next_value(sample_clock, &mut source_data));
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
    fn init_state(&self) -> SoundData {
        Box::new(0)
    }

    fn next_value(&self, n: i32, _state: &mut SoundData) -> (f32, f32) {
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
