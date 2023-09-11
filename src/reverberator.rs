pub mod reverberator {

// This is a very basic reverberator example that uses too few delays, rotations
// and feedback loops and lacks filtering. The hard coded parameters here were
// chosen at random and there hasn't been much experimentation to make it sound
// good.

use std::sync::{Arc, Mutex};

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

use crate::buffer_reader::buffer_reader::BufferReader;
use crate::buffer_writer::buffer_writer::BufferWriter;
use crate::dc::dc::DC;
use crate::mix::mix::Mix;
use crate::multiply::multiply::Multiply;
use crate::rotation_transfer::rotation_transfer::RotationTransfer;
use crate::uneven_delay::uneven_delay::UnevenDelay;

pub struct Reverberator {
    output: DynSoundSource,
    duration: i32,
}

impl Reverberator {
    pub fn new(input: DynSoundSource, gain: f32) -> Self {
        // We assume the sample rate is around 48000 for this code.
        // The usual measure of reverberation time (RT) is the time at which the
        // gain drops by sixty decibels. Add 10% to that for good measure.
        let duration;
        if gain < 1.0 {
            duration = input.duration() + (-3.0 * (1920 + 3360) as f32 / 2.0 / gain.log10() * 1.1) as i32;
        } else {
            duration = input.duration();
        }
        let buffer = Arc::new(Mutex::new(Vec::<(f32,f32)>::new()));
        // Rotate + delaychain of: pi/10, 30ms, pi/10, 55ms, -pi/5, 80ms
        let rotation_1 = RotationTransfer::new(input, std::f32::consts::PI * 0.1);
        let uneven_delay1 = UnevenDelay::new(Box::new(rotation_1), 0, 1440);
        let rotation_2 = RotationTransfer::new(Box::new(uneven_delay1), std::f32::consts::PI * 0.1);
        let uneven_delay2 = UnevenDelay::new(Box::new(rotation_2), 0, 2640);
        let rotation_3 = RotationTransfer::new(Box::new(uneven_delay2), -std::f32::consts::PI * 0.2);
        let uneven_delay3 = UnevenDelay::new(Box::new(rotation_3), 0, 3840);
        // recirculating delay 0.07, 0.04 and gain
        let buffer_reader = BufferReader::new(buffer.clone(), duration);
        let uneven_delay4 = UnevenDelay::new(Box::new(buffer_reader), 3360, 1920);
        let rotation_4 = RotationTransfer::new(Box::new(uneven_delay4), std::f32::consts::PI * 0.3);
        let mut recirculating_gain = Multiply::new();
        recirculating_gain.add(Box::new(DC::new(gain, duration)), 0.0);
        recirculating_gain.add(Box::new(rotation_4), 0.0);
        let mut mix = Mix::new();
        mix.add(Box::new(uneven_delay3));
        mix.add(Box::new(recirculating_gain));
        Reverberator { output: Box::new(BufferWriter::new(Box::new(mix), buffer.clone())), duration: duration }
    }
}

struct ReverberatorData {
    output_data: SoundData,
}

impl SoundSource for Reverberator {
    fn init_state(&self) -> SoundData {
        Box::new( ReverberatorData { output_data: self.output.init_state() } )
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<ReverberatorData>().unwrap();
        self.output.next_value(n, &mut data.output_data)
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let gain = params[1].parse::<f32>().unwrap();
        Box::new(Reverberator::new(input, gain))
    }
}


}
