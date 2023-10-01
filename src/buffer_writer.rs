pub mod buffer_writer {

use std::sync::{Arc, Mutex};

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

#[derive(Clone)]
pub struct BufferWriter {
    source: DynSoundSource,
    buffer: Arc<Mutex<Vec<(f32,f32)>>>
}

impl BufferWriter {
    pub fn new(input: DynSoundSource, buffer: Arc<Mutex<Vec<(f32,f32)>>>) -> Self {
        BufferWriter { source: input, buffer: buffer }
    }
    fn add_sample(&self, n: i32, sample: (f32, f32)) {
        let buffer = &mut self.buffer.lock().unwrap();
        if (buffer.len() as i32) < n {
            buffer.resize(n as usize, (0.0, 0.0));
        }
        if (buffer.len() as i32) == n {
            buffer.push(sample);
        } else {
            buffer[n as usize] = sample;
        }
    }
}

struct BufferWriterData {
    source_data: SoundData
}

impl SoundSource for BufferWriter {
    fn init_state(&self) -> SoundData {
        Box::new(BufferWriterData{source_data: self.source.init_state()})
    }
    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<BufferWriterData>().unwrap();
        let sample = self.source.next_value(n, &mut data.source_data);
        self.add_sample(n, sample.clone());
        sample
    }

    fn duration(&self) -> i32 {
        self.source.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let buffer = reader.get_buffer(&params[1]);
        Box::new(BufferWriter::new(input, buffer))
    }
}

}