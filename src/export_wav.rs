pub mod export_wav {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource};
use crate::pre_render::pre_render::PreRender;
use hound;

pub struct ExportWav {
    buffer: PreRender
}

impl ExportWav {
    fn new(filename: &str, sample_rate: i32, source: DynSoundSource) -> Self {
        println!("Writing file {}...", filename);
        let mut pre_render = PreRender::new(source);
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: sample_rate.try_into().unwrap(),
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = hound::WavWriter::create(filename, spec).unwrap();
        for n in 0..pre_render.duration() {
            let val = pre_render.next_value(n);
            writer.write_sample(val.0).unwrap();
            writer.write_sample(val.1).unwrap();
        }
        println!("done");
        ExportWav { buffer: pre_render }
    }
}

impl SoundSource for ExportWav {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        self.buffer.next_value(n)
    }

    fn duration(&self) -> i32 {
        self.buffer.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let filename = &params[0];
        let source = reader.get_sound(&params[1]);
        Box::new(Self::new(filename, reader.sample_rate, source))
    }
}

}