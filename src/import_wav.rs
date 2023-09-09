pub mod import_wav {

use hound;
use std::io::BufReader;
use std::fs::File;

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

use crate::knob::knob::Knob;
use crate::ramp::ramp::Ramp;
use crate::wavetable::wavetable::{Wavetable, Interpolation};


pub struct ImportWav {
    wavetable: Box<Wavetable>,
}

fn do_read_i32(reader: &mut hound::WavReader<BufReader<File>>, channels: u16, bits_per_sample: u16) -> Vec<(f32, f32)> {
    println!("do_read_i32");
    let mut samples = Vec::<(f32, f32)>::new();
    let mut ch = 0;
    let mut val: (f32, f32) = (0.0, 0.0);
    for s in reader.samples::<i32>() {
        if ch == 0 {
            let sval:i32 = s.unwrap();
            val.0 = sval as f32 / (1_u32 << (bits_per_sample-1)) as f32;
            if channels == 1 {
                val.1 = val.0;
            }
        } else if ch == 1 {
            val.1 = s.unwrap() as f32  / (1_u32 << (bits_per_sample-1)) as f32;
        }
        ch += 1;
        if ch >= channels {
            ch = 0;
            samples.push(val.clone());
            val = (0.0, 0.0);
        }
    }
    println!("Read {} samples", samples.len());
    samples
}

fn do_read_f32(reader: &mut hound::WavReader<BufReader<File>>, channels: u16) -> Vec<(f32, f32)> {
    println!("do_read_f32");
    let mut samples = Vec::<(f32, f32)>::new();
    let mut ch = 0;
    let mut val: (f32, f32) = (0.0, 0.0);
    for s in reader.samples::<f32>() {
        if ch == 0 {
            val.0 = s.unwrap();
            if channels == 1 {
                val.1 = val.0;
            }
        } else if ch == 1 {
            val.1 = s.unwrap();
        }
        ch += 1;
        if ch >= channels {
            ch = 0;
            samples.push(val.clone());
            val = (0.0, 0.0);
        }
    }
    println!("Read {} samples", samples.len());
    samples
}

impl ImportWav {
    pub fn new(filename: &str, sample_rate: i32, interpolation: Interpolation) -> Self {
        println!("Reading file: {}", filename);
        let mut reader = hound::WavReader::open(filename).unwrap();
        let samples: Vec<(f32, f32)>;
        let spec = reader.spec();
        println!("Importing {} {} bit channels, at {} Hz", spec.channels, spec.bits_per_sample, spec.sample_rate);
        match spec.sample_format {
            hound::SampleFormat::Float => samples = do_read_f32(&mut reader, spec.channels),
            hound::SampleFormat::Int => samples = do_read_i32(&mut reader, spec.channels, spec.bits_per_sample),
        }
        println!("Creating wavetable");
        // We will store the file in a wavetable. Wavetables support reading parts of samples at arbitrary sample rates.
        // This function keeps it simple and supplies a sweep function that reads the whole table starting at sample 0
        // and in "real time".

        // This ramp is the sweep function used to convert sample rates.
        // The period of the ramp is taken from the time it takes to play the file in our sample rate.
        let period = samples.len() as f32 / spec.sample_rate as f32 * sample_rate as f32;
        let sweep = Ramp::new(
            Knob::dc(period),
            // The amplitude is set to the number of samples in the file so that one ramp will read the whole file.
            Knob::dc(samples.len() as f32),
            // The duration is set to one ramp time (same as period)
            period.round() as i32);
        let wavetable = Wavetable::from_buffer(samples, Box::new(sweep), interpolation, period.round() as i32);
        ImportWav { wavetable: Box::new(wavetable) }
    }
}

struct ImportWavData {
    wavetable_data: SoundData,
}

impl SoundSource for ImportWav {
    fn init_state(&self) -> SoundData {
        Box::new(ImportWavData { wavetable_data: self.wavetable.init_state() })
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<ImportWavData>().unwrap();
        self.wavetable.next_value(n, &mut data.wavetable_data)
    }

    fn duration(&self) -> i32 {
        self.wavetable.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let filename = &params[0];
        let interpolation: Interpolation;
        match params[1].to_lowercase().as_str() {
            "rounding" => interpolation = Interpolation::Rounding,
            "linear" => interpolation = Interpolation::Linear,
            "cubic" => interpolation = Interpolation::Cubic,
            _ => panic!("Value must be rounding, linear or cubic")
        }
        Box::new(ImportWav::new(filename, reader.sample_rate, interpolation))
    }
}

}
