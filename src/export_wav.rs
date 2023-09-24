pub mod export_wav {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};
use crate::pre_render::pre_render::PreRender;
use hound;

pub struct ExportWav {
    buffer: PreRender
}

impl ExportWav {
    pub fn new(filename: &str, sample_rate: i32, source: DynSoundSource) -> Self {
        println!("Writing file {}...", filename);
        let pre_render = PreRender::new(source);
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: sample_rate.try_into().unwrap(),
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = hound::WavWriter::create(filename, spec).unwrap();
        let mut pre_render_data = pre_render.init_state();
        for n in 0..pre_render.duration() {
            let val = pre_render.next_value(n, &mut pre_render_data);
            writer.write_sample(val.0).unwrap();
            writer.write_sample(val.1).unwrap();
        }
        println!("done");
        ExportWav { buffer: pre_render }
    }
}

struct ExportWavData {
    buffer_data: SoundData
}

impl SoundSource for ExportWav {
    fn init_state(&self) -> SoundData {
        Box::new(ExportWavData{buffer_data: self.buffer.init_state()})
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<ExportWavData>().unwrap();
        self.buffer.next_value(n, &mut data.buffer_data)
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