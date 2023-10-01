pub mod recirculating_delay {

use std::sync::{Arc, Mutex};

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

use crate::dc::dc::DC;
use crate::buffer_reader::buffer_reader::BufferReader;
use crate::buffer_writer::buffer_writer::BufferWriter;
use crate::mix::mix::Mix;
use crate::multiply::multiply::Multiply;
use crate::sequence::sequence::Sequence;


#[derive(Clone)]
pub struct RecirculatingDelay {
    source: DynSoundSource,
}

/*

    input
        |
        |
        V
        Mix <---------------\
        |                   |
        |                   |
        V                   |
        Delay   DC (gain)   |
        |       |           |
        |       |           |
        V       V           |
         Multiply           |
            |               |
            |               |
            \---------------/

    The input to Mix comes from the output of Multiply which also depends on the
    input to Mix. So how to resolve the loop? We need a delay-by-one buffer that
    breaks the dependency - ie it doesn't call Multiply to get it's value but
    instead the last value output from Multiply is made available to it.

    How about a send/receive style model:
    The Send class is a SoundSource that just passes values through but also
    remembers the last value it saw. The Recieve class is a SoundSource that
    fetches it's input from paired Send class but returns (0.0, 0.0) if that
    value hasn't been populated yet. That way it doesn't block when the first
    sample is being made and also can be reset without blowing up.
*/


impl RecirculatingDelay {
    pub fn new(
        input: DynSoundSource,
        delay: i32,
        delay_gain: f32,
        duration: i32
    ) -> Self {
        let buffer = Arc::new(Mutex::new(Vec::<(f32,f32)>::new()));
        let buffer_reader = BufferReader::new(buffer.clone(), duration);
        let mut sequence = Sequence::new();
        sequence.add(delay, Box::new(buffer_reader));
        let mut multiply = Multiply::new();
        let dc_gain = DC::new(delay_gain, duration);
        multiply.add(Box::new(dc_gain), 0.0);
        multiply.add(Box::new(sequence), 0.0);
        let mut mix = Mix::new();
        mix.add(input);
        mix.add(Box::new(multiply));
        let buffer_writer = BufferWriter::new(Box::new(mix), buffer.clone());

        RecirculatingDelay {
            source: Box::new(buffer_writer),
        }
    }
}

struct RecirculatingDelayData {
    source_data: SoundData
}

impl SoundSource for RecirculatingDelay {
    fn init_state(&self) -> SoundData {
        Box::new(RecirculatingDelayData{source_data: self.source.init_state()})
    }
    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<RecirculatingDelayData>().unwrap();
        self.source.next_value(n, &mut data.source_data)
    }

    fn duration(&self) -> i32 {
        self.source.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let delay = params[1].parse::<f32>().unwrap() * reader.sample_rate as f32;
        let delay_gain = params[2].parse::<f32>().unwrap();
        let duration = params[3].parse::<f32>().unwrap() * reader.sample_rate as f32;
        Box::new(RecirculatingDelay::new(input, delay.round() as i32, delay_gain, duration.round() as i32))
    }
}

}