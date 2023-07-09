use anyhow;
use std::sync::{Arc, Mutex, Condvar};
use std::env;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, SizedSample};

mod traits;
mod pure_tone;
mod midi_notes;
mod tremolo;
mod ding_envelope;
mod sequence;
mod mix;
mod songs;
mod instruments;
mod generative_waveform;
mod square;
mod triangle;
mod saw;
mod dc;
mod knob;
mod lfo;
mod envelope;
mod multiply;
mod low_pass_filter;
mod pre_render;
mod sine;

use traits::traits::{DynSoundSource, DynInstrument};

// todo make command line args select the song to play
fn get_song(songname: &String, instrument_name: &String, sample_rate: i32) -> DynSoundSource {
    let instrument: DynInstrument;
    if instrument_name == "vibraphone" {
        instrument = Box::new(instruments::vibraphone::vibraphone::Vibraphone::new(sample_rate));
    } else if instrument_name == "kick" {
        instrument = Box::new(instruments::kick::kick::Kick::new(sample_rate));
    } else if instrument_name == "square_ding" {
        instrument = Box::new(instruments::square_ding::square_ding::SquareDing::new(sample_rate));
    } else if instrument_name == "triangle_ding" {
        instrument = Box::new(instruments::triangle_ding::triangle_ding::TriangleDing::new(sample_rate));
    } else if instrument_name == "saw_ding" {
        instrument = Box::new(instruments::saw_ding::saw_ding::SawDing::new(sample_rate));
    } else if instrument_name == "experiment" {
        instrument = Box::new(instruments::experiment::experiment::Experiment::new(sample_rate));
    } else if instrument_name == "uphonium" {
        instrument = Box::new(instruments::uphonium::uphonium::Uphonium::new(sample_rate));
    } else {
        panic!("Unkown instrument: '{}'", songname)
    }
    if songname == "arpeggios" {
        songs::arpeggios::arpeggios::arpeggios(sample_rate, instrument)
    } else if songname == "long_note" {
        songs::long_note::long_note::long_note(sample_rate, instrument)
    } else if songname == "beats" {
        songs::beats::beats::beats(sample_rate, instrument)
    } else {
        panic!("Unkown song: '{}'", songname)
    }
}


fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => run::<f32>(&device, &config.into()),
        SampleFormat::I16 => run::<i16>(&device, &config.into()),
        SampleFormat::U16 => run::<u16>(&device, &config.into()),
        sample_format => panic!("Unsupported sample format '{sample_format}'")
    }
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Specify name of song and instrument: arpeggios vibraphone");
    }
    let songname = &args[1];
    let instrument_name = &args[2];

    let sample_rate = config.sample_rate.0 as i32;
    let mut song = get_song(songname, instrument_name, sample_rate);
    let channels = config.channels as usize;
    let mut sample_clock = 0i32;
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    let mut next_value = move || -> (f32, f32) {
        let (lock, cvar) = &*pair2;
        sample_clock = sample_clock + 1;
        if sample_clock > (*song).duration() {
            let mut done = lock.lock().unwrap();
            *done = true;
            cvar.notify_one();
            (0.0, 0.0)
        } else {
            (*song).next_value(sample_clock)
        }
    };

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
        None
    )?;

    fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> (f32, f32))
    where T: Sample + FromSample<f32>,
    {

        for frame in output.chunks_mut(channels) {
            let nexts = next_sample();
            for sample in frame.iter_mut() {
                *sample = T::from_sample(0.0);
            }
            frame[0] = T::from_sample(nexts.0);
            frame[1] = T::from_sample(nexts.1);
        }
    }

    stream.play()?;

    let (lock, cvar) = &*pair;
    let mut done = lock.lock().unwrap();
    while !*done {
        done = cvar.wait(done).unwrap();
    }

    Ok(())
}
