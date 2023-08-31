pub mod wavetable {

use std::sync::{Arc, Mutex};

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

pub enum Interpolation {
    Rounding,
    Linear,
    Cubic,
}

pub struct Wavetable {
    table: Vec::<(f32,f32)>,
    sweep: DynSoundSource,
    interpolation: Interpolation,
    duration: i32
}

impl Wavetable {
    pub fn new(table: DynSoundSource, sweep: DynSoundSource, interpolation: Interpolation, duration: i32) -> Self {
        let mut buf = Vec::<(f32, f32)>::new();
        let mut sample_clock = 0i32;
        let table_duration = table.duration();
        let table_state = table.init_state();
        while sample_clock < table_duration {
            buf.push(table.next_value(sample_clock, table_state));
            sample_clock += 1;
        }
        Wavetable { table: buf, sweep: sweep, interpolation: interpolation, duration: duration }
    }
}

fn wrap_x(mut x: i32, table_len: i32) -> usize {
    while x >= table_len {
        x -= table_len;
    }
    while x < 0 {
        x += table_len;
    }
    x as usize
}

pub struct WavetableState {
    sweep_state: SoundData
}

const SWEEP_STATE: usize = 0;

impl SoundState for WavetableState {
    fn get_sound_state(&self, key: usize) -> SoundData {
        match key {
            SWEEP_STATE => self.sweep_state,
            _ => panic!("WavetableState unknown key {}", key)
        }
    }
}

impl SoundSource for Wavetable {
    fn init_state(&self) -> SoundData {
        Box::new(WavetableState { sweep_state: self.sweep.init_state() })
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = state.downcast_mut::<MyData>().unwrap();
        let sweep_value = self.sweep.next_value(n, data.get_sound_state(SWEEP_STATE)).0;
        if sweep_value.floor() >= 0.0 && (sweep_value.ceil() as usize) < self.table.len() {
            let output0: f32;
            let output1: f32;
            match self.interpolation {
                Interpolation::Rounding => {
                    let x0 = sweep_value.round() as usize;
                    let y = &self.table;
                    (output0, output1) = y[x0];
                },
                Interpolation::Linear => {
                    let x0 = sweep_value.floor() as usize;
                    let y = &self.table;
                    let y1 = y[wrap_x(x0 as i32 + 1, y.len() as i32)]; // y[x0 + 1]
                    output0 = y[x0].0 + (y1.0 - y[x0].0) * (sweep_value - x0 as f32);
                    output1 = y[x0].1 + (y1.1 - y[x0].1) * (sweep_value - x0 as f32);
                },
                Interpolation::Cubic => {
                    let x0 = sweep_value.floor() as usize;
                    let y = &self.table;
                    let y1 = y[wrap_x(x0 as i32 + 1, y.len() as i32)]; // y[x0 + 1]
                    let y2 = y[wrap_x(x0 as i32 + 2, y.len() as i32)]; // y[x0 + 2]
                    let ym1 = y[wrap_x(x0 as i32 - 1, y.len() as i32)]; // y[x0 - 1]
                    let f = sweep_value - x0 as f32;
                    output0 = -f * (f - 1.0) * (f - 2.0) / 6.0 * ym1.0
                        + (f + 1.0) * (f - 1.0) * (f - 2.0) / 2.0 * y[x0].0
                        -(f + 1.0) * f * (f - 2.0) / 2.0 * y1.0
                        + (f + 1.0) * f * (f - 1.0) / 6.0 * y2.0;
                    output1 = -f * (f - 1.0) * (f - 2.0) / 6.0 * ym1.1
                        + (f + 1.0) * (f - 1.0) * (f - 2.0) / 2.0 * y[x0].1
                        -(f + 1.0) * f * (f - 2.0) / 2.0 * y1.1
                        + (f + 1.0) * f * (f - 1.0) / 6.0 * y2.1;
                }
            }
            (output0, output1)
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
        let interpolation: Interpolation;
        match params[2].to_lowercase().as_str() {
            "rounding" => interpolation = Interpolation::Rounding,
            "linear" => interpolation = Interpolation::Linear,
            "cubic" => interpolation = Interpolation::Cubic,
            _ => panic!("Value must be rounding, linear or cubic")
        }
        let duration = params[3].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(Wavetable::new(table, sweep, interpolation, duration.round() as i32))
    }
}

}