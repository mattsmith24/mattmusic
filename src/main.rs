use anyhow;
use std::sync::{Arc, Mutex, Condvar};
use clap::{Parser, ValueEnum};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, SizedSample};

mod buffer_reader;
mod buffer_writer;
mod cauchy_transfer;
mod clip;
mod cos_transfer;
mod db2amp;
mod dc;
mod delay_line;
mod envelope;
mod export_wav;
mod gaussian_transfer;
mod generative_waveform;
mod hann_window;
mod import_wav;
mod knob;
mod midi_notes;
mod midi2freq;
mod mix;
mod multiply;
mod noise;
mod oscillator;
mod pitch_shift;
mod pre_render;
mod pulse_train;
mod ramp;
mod read_song;
mod recirculating_delay;
mod reverberator;
mod rotation_transfer;
mod saw;
mod sequence;
mod sine;
mod square;
mod time_box;
mod traits;
mod triangle;
mod uneven_delay;
mod wavetable;

mod filters;
mod instruments;
mod songs;


use traits::traits::{DynSoundSource, DynInstrument};
use read_song::read_song::read_song;
use wavetable::wavetable::Interpolation;
use import_wav::import_wav::ImportWav;

// todo make command line args select the song to play
fn get_song(songname: &Option<Song>, instrument_name: &Option<InstrumentName>, sample_rate: i32) -> DynSoundSource {
    let instrument: DynInstrument;
    match instrument_name {
    Some(InstrumentName::Vibraphone) => {
        instrument = Box::new(instruments::vibraphone::vibraphone::Vibraphone::new(sample_rate)); }
    Some(InstrumentName::Kick) => {
        instrument = Box::new(instruments::kick::kick::Kick::new(sample_rate)); }
    Some(InstrumentName::SquareDing) => {
        instrument = Box::new(instruments::square_ding::square_ding::SquareDing::new(sample_rate)); }
    Some(InstrumentName::TriangleDing) => {
        instrument = Box::new(instruments::triangle_ding::triangle_ding::TriangleDing::new(sample_rate)); }
    Some(InstrumentName::SawDing) => {
        instrument = Box::new(instruments::saw_ding::saw_ding::SawDing::new(sample_rate)); }
    Some(InstrumentName::Experiment) => {
        instrument = Box::new(instruments::experiment::experiment::Experiment::new(sample_rate)); }
    Some(InstrumentName::Uphonium) => {
        instrument = Box::new(instruments::uphonium::uphonium::Uphonium::new(sample_rate)); },
    &None => todo!()
    }
    match songname {
    Some(Song::Arpeggios) => {
        songs::arpeggios::arpeggios::arpeggios(sample_rate, instrument) }
    Some(Song::LongNote) => {
        songs::long_note::long_note::long_note(sample_rate, instrument) }
    Some(Song::Beats) => {
        songs::beats::beats::beats(sample_rate, instrument) }
    Some(Song::TwoNotes) => {
        songs::two_notes::two_notes::two_notes(sample_rate, instrument) }
    Some(Song::ManyNotes) => {
        songs::many_notes::many_notes::many_notes(sample_rate, instrument) },
    &None => todo!()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Song {
    Arpeggios,
    LongNote,
    Beats,
    TwoNotes,
    ManyNotes
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum InstrumentName {
    Vibraphone,
    Kick,
    SquareDing,
    TriangleDing,
    SawDing,
    Experiment,
    Uphonium
}


/// Mattmusic - a code driven sythesiser
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Play using built-in instrument
    #[arg(value_enum, short, long, requires="song")]
    instrument: Option<InstrumentName>,
    /// Play built-in song
    #[arg(value_enum, short, long, requires="instrument")]
    song: Option<Song>,
    /// Read song and instrument from file (will ignore --song and --instrument)
    #[arg(short, long)]
    file: Option<String>,
    /// Play a wav file
    #[arg(short, long)]
    wavfile: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => run::<f32>(&args, &device, &config.into()),
        SampleFormat::I16 => run::<i16>(&args, &device, &config.into()),
        SampleFormat::U16 => run::<u16>(&args, &device, &config.into()),
        sample_format => panic!("Unsupported sample format '{sample_format}'")
    }
}

fn run<T>(args: &Args, device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as i32;
    println!("Output sample rate is {} Hz", sample_rate);
    let song;
    if let Some(filename) = &args.file {
        song = read_song(&filename, sample_rate);
    } else if let Some(filename) = &args.wavfile {
        song = Box::new(ImportWav::new(filename, sample_rate, Interpolation::Cubic));
    } else {
        song = get_song(&args.song, &args.instrument, sample_rate);
    }
    let mut song_state = song.init_state();
    let channels = config.channels as usize;
    let mut sample_clock = 0i32;
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    let mut next_value = move || -> (f32, f32) {
        let (lock, cvar) = &*pair2;
        sample_clock = sample_clock + 1;
        if sample_clock > song.duration() {
            let mut done = lock.lock().unwrap();
            *done = true;
            cvar.notify_one();
            (0.0, 0.0)
        } else {
            song.next_value(sample_clock, &mut song_state)
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

    println!("Playing");
    stream.play()?;

    let (lock, cvar) = &*pair;
    let mut done = lock.lock().unwrap();
    while !*done {
        done = cvar.wait(done).unwrap();
    }

    Ok(())
}
