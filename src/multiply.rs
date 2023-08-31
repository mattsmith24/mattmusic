pub mod multiply {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};
use crate::dc::dc::DC;

pub struct MultiplyInput {
    source: DynSoundSource,
    offset: f32
}

pub struct Multiply {
    inputs: Vec<MultiplyInput>
}

impl Multiply {
    pub fn new() -> Self {
        Multiply{ inputs: Vec::<MultiplyInput>::new() }
    }

    pub fn add(&mut self, source: DynSoundSource, offset: f32) -> &mut Multiply {
        self.inputs.push(MultiplyInput { source:source, offset: offset });
        self
    }
}

pub struct MultiplyState {
    inputs: Vec<SoundData>
}

impl SoundSource for Multiply {
    fn init_state(&self) -> SoundData {
        let mut data = MultiplyState { inputs: Vec::<SoundData>::new() };
        for input in &self.inputs {
            data.inputs.push(input.source.init_state())
        }
        Box::new(data)
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<MultiplyState>().unwrap();
        let mut res1: f32 = 1.0;
        let mut res2: f32 = 1.0;
        let mut idx = 0;
        for minput in &self.inputs {
            let (v1, v2) = minput.source.next_value(n, &mut data.inputs[idx]);
            res1 *= v1 + minput.offset;
            res2 *= v2 + minput.offset;
            idx += 1;
        }
        (res1, res2)
    }
    fn duration(&self) -> i32 {
        let mut duration: i32 = 0;
        for minput in self.inputs.iter() {
            duration = duration.max(minput.source.duration());
        }
        duration
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let mut multiply = Multiply::new();
        for param in params {
            println!("Multiply::from_yaml(param: {})", param);
            let parts: Vec<_> = param.split(" ").collect();
            // If the first token is 'dc' then we expect the following to be the value and duration
            // Otherwise we expect to see a dc offset and a source name
            if parts[0] == "dc" {
                let val = parts[1].parse::<f32>().unwrap();
                let duration = parts[2].parse::<f32>().unwrap() * reader.sample_rate as f32;
                let source = Box::new(DC::new(val, duration.round() as i32));
                multiply.add(source, 0.0);
            } else {
                let dc_offset = parts[0].parse::<f32>().unwrap();
                let source_name = parts[1];
                let source = reader.get_sound(source_name);
                multiply.add(source, dc_offset);
            }
        }
        Box::new(multiply)
    }
}

}