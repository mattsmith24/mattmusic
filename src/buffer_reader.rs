pub mod buffer_reader {

use std::sync::{Arc, Mutex};

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

pub struct BufferReader {
    buffer: Arc<Mutex<Vec<(f32,f32)>>>,
    duration: i32,
}

impl BufferReader {
    pub fn new(buffer: Arc<Mutex<Vec<(f32,f32)>>>, duration: i32) -> Self {
        BufferReader { buffer: buffer, duration: duration }
    }
    fn get_sample(&self, n: i32) -> (f32, f32) {
        let buffer = &mut self.buffer.lock().unwrap();
        if buffer.len() as i32 <= n || n < 0 {
            (0.0, 0.0)
        } else {
            buffer[n as usize].clone()
        }
    }
}

impl SoundSource for BufferReader {
    fn init_state(&self) -> SoundData {
        {
            let buffer = &mut self.buffer.lock().unwrap();
            buffer.clear();
        }
        Box::new(0)
    }
    fn next_value(&self, n: i32, _state: &mut SoundData) -> (f32, f32) {
        self.get_sample(n)
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let buffer = reader.get_buffer(&params[0]);
        let duration = params[1].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(BufferReader::new(buffer, duration.round() as i32))
    }
}

}