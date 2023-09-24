pub mod pitch_shift {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

use crate::knob::knob::Knob;

use crate::cos_transfer::cos_transfer::CosTransfer;
use crate::dc::dc::DC;
use crate::delay_line::delay_line::DelayLine;
use crate::mix::mix::Mix;
use crate::multiply::multiply::Multiply;
use crate::pre_render::pre_render::PreRender;
use crate::ramp::ramp::Ramp;
use crate::sequence::sequence::Sequence;

pub struct PitchShift {
    output: DynSoundSource,
}
// This pitch shifter is from The Theory and Techniques of Electronic Music by
// Miller Puckette (ch 7.9). It uses the doppler effect of a delay to shift the
// pitch. By using two circuits out of phase and an envelope, it can
// continuously generate overlapping fragments of doppler shifted audio. One
// thing to note is that window size 's' can be heard as an echo if it's larger
// than 50ms. Also the frequency of the ramp becomes very audible above 5 Hz.
// There is an equation to work out the transposition as a factor between 0 and
// 1 using 1 - f*s/R.

// Two pitch shift circuits are created based off the following diagram
// but the second one is phase shifted by 180 degrees.
//
//                                f
//                                |
//      IN                        V
//       |                    1|------|
//       V                     | Ramp |
//     |---|                  0|------|
//     |   |                       |
//     |   |         +------------+
//     |   |         |            |
//     |   |         V            |
//     |   |       |---|          |
//     |   |       | X |<-- s     |
//     |   |       |---|          V
//     |   |         |          |---|
//     |   |         V          | X |<--N
//     |   | delay |---|        |---|
//     |   |<------| + |<-- d0    |
//     |   |       |---|          |
//     |   |                      V
//     |   |                 1|--------|
//     |---|                  | Window |
//       |                   0|--------|
//       V                    0   |    N
//     |---|                      |
//     | X |<---------------------+
//     |---|
//       |
//       V
//      OUT
fn new_circuit(input: DynSoundSource, base_delay: i32, window_size: i32, freq: f32, phase: f32) -> DynSoundSource {
    let duration = input.duration() + base_delay + window_size;
    let direction = freq.signum();
    let period = (1.0 / freq.abs()).round() as i32;
    let ramp_delay = (phase / (2.0 * std::f32::consts::PI) / freq.abs()).round() as i32;

    // Delay line
    let mut ramp_sequence1 = Sequence::new();
    if direction > 0.0 {
        let ramp1 = Ramp::new(Knob::dc(period as f32), Knob::dc(1.0), duration);
        ramp_sequence1.add(ramp_delay, Box::new(ramp1));
    } else {
        let ramp1 = Ramp::new(Knob::dc(period as f32), Knob::dc(-1.0), duration);
        // We need to add 1 to ramp because it't current going from 0 to -1 and
        // we want it to go from 1 to 0.
        let mut ramp_offset = Multiply::new();
        ramp_offset.add(Box::new(ramp1), 1.0);
        ramp_sequence1.add(ramp_delay, Box::new(ramp_offset));
    }
    let mut delay_window_multiplier = Multiply::new();
    delay_window_multiplier.add(Box::new(ramp_sequence1), 0.0);
    delay_window_multiplier.add(Box::new(DC::new(window_size as f32, duration)), 0.0);
    let mut delay_adder = Mix::new();
    delay_adder.add(Box::new(DC::new(base_delay as f32, duration)));
    delay_adder.add(Box::new(delay_window_multiplier));
    let delay_line = DelayLine::new(input, Knob::new(Box::new(delay_adder)));

    // Window envelope
    let ramp2 = Ramp::new(Knob::dc(period as f32), Knob::dc(1.0), duration);
    let mut ramp_sequence2 = Sequence::new();
    ramp_sequence2.add(ramp_delay, Box::new(ramp2));
    // Use multiplier to add a dc offset to the ramp
    let mut window_envelope_multiplier = Multiply::new();
    window_envelope_multiplier.add(Box::new(ramp_sequence2), -0.5);
    window_envelope_multiplier.add(Box::new(DC::new(0.5, duration)), 0.0);
    let envelope = CosTransfer::new(Box::new(window_envelope_multiplier));

    // multiply delay line by envelope window
    let mut output_multiplier = Multiply::new();
    output_multiplier.add(Box::new(delay_line), 0.0);
    output_multiplier.add(Box::new(envelope), 0.0);
    Box::new(output_multiplier)
}

impl PitchShift {
    pub fn new(input: DynSoundSource, base_delay: i32, window_size: i32, freq: f32) -> Self {
        // We need to use the input twice so prerender it and clone.
        let input0 = PreRender::new(input);
        let input1 = input0.clone();
        let circuit0 = new_circuit(Box::new(input0), base_delay, window_size, freq, 0.0);
        let circuit1 = new_circuit(Box::new(input1), base_delay, window_size, freq, std::f32::consts::PI);
        let mut mix = Mix::new();
        mix.add(circuit0);
        mix.add(circuit1);
        PitchShift { output: Box::new(mix) }
    }
}

struct PitchShiftData {
    output_data: SoundData,
}

impl SoundSource for PitchShift {
    fn init_state(&self) -> SoundData {
        Box::new(PitchShiftData { output_data: self.output.init_state() })
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<PitchShiftData>().unwrap();
        self.output.next_value(n, &mut data.output_data)
    }

    fn duration(&self) -> i32 {
        self.output.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let base_delay = params[1].parse::<f32>().unwrap() * reader.sample_rate as f32;
        let window_size = params[2].parse::<f32>().unwrap() * reader.sample_rate as f32;
        let freq = params[3].parse::<f32>().unwrap() / reader.sample_rate as f32;
        Box::new(PitchShift::new(input, base_delay.round() as i32, window_size.round() as i32, freq))
    }
}

}
