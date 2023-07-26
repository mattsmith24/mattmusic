pub mod wavetable {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource};

pub struct Wavetable {
    table: Vec::<(f32,f32)>,
    sweep: DynSoundSource,
    duration: i32
}

impl Wavetable {
    pub fn new(mut table: DynSoundSource, sweep: DynSoundSource, duration: i32) -> Self {
        let mut buf = Vec::<(f32, f32)>::new();
        let mut sample_clock = 0i32;
        let table_duration = (*table).duration();
        while sample_clock < table_duration {
            buf.push((*table).next_value(sample_clock));
            sample_clock += 1;
        }
        Wavetable { table: buf, sweep: sweep, duration: duration }
    }
}

impl SoundSource for Wavetable {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        let sweep_value = self.sweep.next_value(n).0;
        let idx = sweep_value.round() as usize;
        if idx < self.table.len() {
            self.table[idx]
        } else {
            (0.0, 0.0)
        }
    }
    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let table = reader.get_sound(&params[0]);
        let sweep = reader.get_sound(&params[1]);
        let duration = params[2].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(Wavetable::new(table, sweep, duration.round() as i32))
    }
}

}