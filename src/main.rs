use anyhow;
use std::sync::{Arc, Mutex, Condvar};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, SizedSample};


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

trait SoundSource {
    fn next_value(&mut self, t: f32) -> (f32, f32);
    fn is_done(&self, t: f32) -> bool;
}
type DynSoundSource = Box<dyn SoundSource + Send + Sync>;

struct PureTone {
    freq: f32,
    gain: f32,
    duration: f32,
    last_time: f32,
}

impl PureTone {
    fn new(
        freq: f32,
        gain: f32,
        duration: f32
    ) -> Self {
        if freq <= 0.0 {
            panic!("freq must be greater than 0.0");
        }
        PureTone{
            freq: freq,
            gain: gain,
            duration: duration,
            last_time: 0.0
        }
    }
}

impl SoundSource for PureTone {
    fn next_value(&mut self, t: f32) -> (f32, f32) {
        if self.is_done(t) {
            (0.0, 0.0)
        } else {
            let val = (t * self.freq * 2.0 * std::f32::consts::PI).sin() * self.gain;
            self.last_time = t;
            (val, val)
        }
    }

    fn is_done(&self, t: f32) -> bool {
        t >= self.duration
    }
}

const MIDI_NOTES : [(u8, &'static str, u8, f32, f32); 99] = [
    (21,"A",0,27.5,27.00),
    (22,"A#",0,29.1353,28.61),
    (23,"B",0,30.8677,30.31),
    (24,"C",1,32.7032,32.11),
    (25,"C#",1,34.6479,34.02),
    (26,"D",1,36.7081,36.04),
    (27,"D#",1,38.8909,38.18),
    (28,"E",1,41.2035,40.45),
    (29,"F",1,43.6536,42.86),
    (30,"F#",1,46.2493,45.41),
    (31,"G",1,48.9995,48.11),
    (32,"G#",1,51.913,50.97),
    (33,"A",1,55.0,54.00),
    (34,"A#",1,58.2705,57.21),
    (35,"B",1,61.7354,60.61),
    (36,"C",2,65.4064,64.22),
    (37,"C#",2,69.2957,68.04),
    (38,"D",2,73.4162,72.08),
    (39,"D#",2,77.7817,76.37),
    (40,"E",2,82.4069,80.91),
    (41,"F",2,87.3071,85.72),
    (42,"F#",2,92.4986,90.82),
    (43,"G",2,97.9989,96.22),
    (44,"G#",2,103.826,101.94),
    (45,"A",2,110.0,108.00),
    (46,"A#",2,116.541,114.42),
    (47,"B",2,123.471,121.23),
    (48,"C",3,130.813,128.43),
    (49,"C#",3,138.591,136.07),
    (50,"D",3,146.832,144.16),
    (51,"D#",3,155.563,152.74),
    (52,"E",3,164.814,161.82),
    (53,"F",3,174.614,171.44),
    (54,"F#",3,184.997,181.63),
    (55,"G",3,195.998,192.43),
    (56,"G#",3,207.652,203.88),
    (57,"A",3,220.0,216.00),
    (58,"A#",3,233.082,228.84),
    (59,"B",3,246.942,242.45),
    (60,"C",4,261.626,256.87),
    (61,"C#",4,277.183,272.14),
    (62,"D",4,293.665,288.33),
    (63,"D#",4,311.127,305.47),
    (64,"E",4,329.628,323.63),
    (65,"F",4,349.228,342.88),
    (66,"F#",4,369.994,363.27),
    (67,"G",4,391.995,384.87),
    (68,"G#",4,415.305,407.75),
    (69,"A",4,440.0,432.00),
    (70,"A#",4,466.164,457.69),
    (71,"B",4,493.883,484.90),
    (72,"C",5,523.251,513.74),
    (73,"C#",5,554.365,544.29),
    (74,"D",5,587.33,576.65),
    (75,"D#",5,622.254,610.94),
    (76,"E",5,659.255,647.27),
    (77,"F",5,698.456,685.76),
    (78,"F#",5,739.989,726.53),
    (79,"G",5,783.991,769.74),
    (80,"G#",5,830.609,815.51),
    (81,"A",5,880.0,864.00),
    (82,"A#",5,932.328,915.38),
    (83,"B",5,987.767,969.81),
    (84,"C",6,1046.5,1027.47),
    (85,"C#",6,1108.73,1088.57),
    (86,"D",6,1174.66,1153.30),
    (87,"D#",6,1244.51,1221.88),
    (88,"E",6,1318.51,1294.54),
    (89,"F",6,1396.91,1371.51),
    (90,"F#",6,1479.98,1453.07),
    (91,"G",6,1567.98,1539.47),
    (92,"G#",6,1661.22,1631.01),
    (93,"A",6,1760.0,1728.00),
    (94,"A#",6,1864.66,1830.75),
    (95,"B",6,1975.53,1939.61),
    (96,"C",7,2093.0,2054.95),
    (97,"C#",7,2217.46,2177.14),
    (98,"D",7,2349.32,2306.60),
    (99,"D#",7,2489.02,2443.76),
    (100,"E",7,2637.02,2589.07),
    (101,"F",7,2793.83,2743.03),
    (102,"F#",7,2959.96,2906.14),
    (103,"G",7,3135.96,3078.95),
    (104,"G#",7,3322.44,3262.03),
    (105,"A",7,3520.0,3456.00),
    (106,"A#",7,3729.31,3661.50),
    (107,"B",7,3951.07,3879.23),
    (108,"C",8,4186.01,4109.90),
    (109,"C#",8,4434.92,4354.29),
    (110,"D",8,4698.63,4613.21),
    (111,"D#",8,4978.03,4887.52),
    (112,"E",8,5274.04,5178.15),
    (113,"F",8,5587.65,5486.06),
    (114,"F#",8,5919.91,5812.28),
    (115,"G",8,6271.93,6157.89),
    (116,"G#",8,6644.88,6524.06),
    (117,"A",8,7040.0,6912.00),
    (118,"A#",8,7458.62,7323.01),
    (119,"B",8,7902.13,7758.46),
];

#[allow(dead_code)] const MIDI_OFFSET_A: u8 = 9u8;
#[allow(dead_code)] const MIDI_OFFSET_A_SHARP: u8 = 10u8;
#[allow(dead_code)] const MIDI_OFFSET_B_FLAT: u8 = 10u8;
#[allow(dead_code)] const MIDI_OFFSET_B: u8 = 11u8;
#[allow(dead_code)] const MIDI_OFFSET_C: u8 = 0u8;
#[allow(dead_code)] const MIDI_OFFSET_C_SHARP: u8 = 1u8;
#[allow(dead_code)] const MIDI_OFFSET_D_FLAT: u8 = 1u8;
#[allow(dead_code)] const MIDI_OFFSET_D: u8 = 2u8;
#[allow(dead_code)] const MIDI_OFFSET_D_SHARP: u8 = 3u8;
#[allow(dead_code)] const MIDI_OFFSET_E_FLAT: u8 = 3u8;
#[allow(dead_code)] const MIDI_OFFSET_E: u8 = 4u8;
#[allow(dead_code)] const MIDI_OFFSET_F: u8 = 5u8;
#[allow(dead_code)] const MIDI_OFFSET_F_SHARP: u8 = 6u8;
#[allow(dead_code)] const MIDI_OFFSET_G_FLAT: u8 = 6u8;
#[allow(dead_code)] const MIDI_OFFSET_G: u8 = 7u8;
#[allow(dead_code)] const MIDI_OFFSET_G_SHARP: u8 = 8u8;
#[allow(dead_code)] const MIDI_OFFSET_A_FLAT: u8 = 8u8;
const MIDI_OFFSET_OCTAVE_1: u8 = 3u8;

fn midi_octave_offset(oct: u8) -> i8 {
    MIDI_OFFSET_OCTAVE_1 as i8 - 12 + oct as i8 * 12
}

fn note2freq(octave: u8, pitch: u8) -> f32 {
    let offset: i8 = midi_octave_offset(octave) + pitch as i8;
    if offset < 0 {
        panic!("Note too low {} {}", octave, pitch)
    }
    MIDI_NOTES[offset as usize].3
}

struct Tremolo
{
    freq: f32,
    depth: f32,
    source: DynSoundSource,
}

impl Tremolo {
    fn new(
        freq: f32,
        depth: f32,
        source: DynSoundSource,
    ) -> Self {
        if depth > 1.0 || depth < 0.0 {
            panic!("depth must be between 0.0 and 1.0");
        }
        if freq <= 0.0 {
            panic!("freq must be greater than 0.0");
        }
        Tremolo{
            freq: freq,
            depth: depth,
            source: source,
        }
    }
}

impl SoundSource for Tremolo {
    fn next_value(&mut self, t: f32) -> (f32, f32) {
        let source_val = (*self.source).next_value(t);
        let tremolo_gain = 1.0 - ((t * self.freq * 2.0 * std::f32::consts::PI).sin() + 1.0) * 0.5 * self.depth;
        (source_val.0 * tremolo_gain, source_val.1 * tremolo_gain)
    }

    fn is_done(&self, t: f32) -> bool {
        (*self.source).is_done(t)
    }
}

struct DingEnvelope {
    decay: f32,
    duration: f32,
    source: DynSoundSource,
}

impl DingEnvelope {
    fn new(
        decay: f32,
        duration: f32,
        source: DynSoundSource
    ) -> Self {
        DingEnvelope{
            decay: decay,
            duration: duration,
            source: source
        }
    }
}

impl SoundSource for DingEnvelope {
    fn next_value(&mut self, t: f32) -> (f32, f32) {
        let source_val = (*self.source).next_value(t);
        let mut gain;
        const IMPULSE: f32 = 0.05;
        const FALLOFF: f32 = 0.1;
        if t < IMPULSE {
            gain = 1.0;
        } else if t < FALLOFF {
            // IMPULSE < t < FALLOFF, gain from 1.0 to 0.5
            gain = 1.0 + (t - IMPULSE) * -0.5 / (FALLOFF - IMPULSE);
        } else {
            // FALLOFF < t < decay, gain from 0.5 to 0
            gain = 0.5 + (t - FALLOFF) * -0.5 / (self.decay - FALLOFF);
        }
        const LIFT: f32 = 0.001;
        if t > self.duration - LIFT {
            // ramp down to end of note to avoid discontinuity
            gain *= 1.0 - (t - (self.duration - LIFT)) / LIFT;
        }
        if gain < 0.0 || t > self.duration {
            gain = 0.0;
        }
        (source_val.0 * gain, source_val.1 * gain)
    }

    fn is_done(&self, t: f32) -> bool {
        t > self.decay || t > self.duration || (*self.source).is_done(t)
    }
}

struct SequenceMember {
    sound_source: DynSoundSource,
    start_time: f32
}

struct Sequence {
    notes: Vec<SequenceMember>
}

impl Sequence {
    fn new(bpm: f32, mut notes: Vec<DynSoundSource>) -> Self {
        let mut vec = Vec::<SequenceMember>::new();
        let mut t_idx: f32 = 0.0;
        for note in notes.drain(..) {
            vec.push( SequenceMember { sound_source: note, start_time: t_idx } );
            t_idx += 60.0 / bpm;
        }
        Sequence{ notes: vec }
    }
}

impl SoundSource for Sequence {
    fn next_value(&mut self, t: f32) -> (f32, f32) {
        let mut res1: f32 = 0.0;
        let mut res2: f32 = 0.0;
        for note in self.notes.iter_mut() {
            if t >= note.start_time {
                let (v1, v2) = (*note.sound_source).next_value(t - note.start_time);
                res1 += v1;
                res2 += v2;
            }
        }
        (res1, res2)
    }
    fn is_done(&self, t: f32) -> bool {
        let note = &self.notes[self.notes.len() - 1];
        (*note.sound_source).is_done(t - note.start_time)
    }
}

fn vibraphone(octave: u8, pitch: u8, duration: f32) -> DynSoundSource {
    Box::new(
        Tremolo::new(
            5.0,
            0.5,
            Box::new(
                DingEnvelope::new(
                    2.0,
                    duration,
                    Box::new(
                        PureTone::new(note2freq(octave, pitch), 0.5, duration * 2.0)
                    )
                )
            )
        )
    )
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let mut sample_clock = 0f32;
    let bpm = 160.0;
    let note_duration = 60.0 / bpm * 0.9;
    let mut vec = Vec::<DynSoundSource>::new();
    vec.push(vibraphone(4, MIDI_OFFSET_C, note_duration));
    vec.push(vibraphone(4, MIDI_OFFSET_E, note_duration));
    vec.push(vibraphone(4, MIDI_OFFSET_G, note_duration));
    vec.push(vibraphone(5, MIDI_OFFSET_C, 5.0));
    let mut sound_source = Sequence::new(bpm, vec);
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    let mut next_value = move || -> (f32, f32) {
        let (lock, cvar) = &*pair2;
        sample_clock = sample_clock + 1.0;
        let t: f32 = sample_clock / sample_rate;
        if sound_source.is_done(t) {
            let mut done = lock.lock().unwrap();
            *done = true;
            cvar.notify_one();
            (0.0, 0.0)
        } else {
            sound_source.next_value(t)
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
            let value: (T, T) = (T::from_sample(nexts.0), T::from_sample(nexts.1));
            for sample in frame.iter_mut() {
                *sample = T::from_sample(0.0);
            }
            frame[0] = value.0;
            frame[1] = value.1;
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
