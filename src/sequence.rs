pub mod sequence {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource};


struct SequenceMember {
    sound_source: DynSoundSource,
    start_time: i32,
    is_playing: bool,
    playing_start_time: i32
}

pub struct Sequence {
    notes: Vec<SequenceMember>,
    repeat: u32,
    duration: i32
}

impl Sequence {
    pub fn new() -> Self {
        Sequence{ notes: Vec::<SequenceMember>::new(), repeat: 1, duration: 0 }
    }

    // Add notes into the sequence at arbitrary time offsets
    pub fn add(&mut self, start_time: i32, note: DynSoundSource) -> &mut Sequence {
        self.notes.push( SequenceMember { sound_source: note, start_time: start_time, is_playing: false, playing_start_time: 0 } );
        self.duration = self.calculate_duration();
        self
    }

    // Use new for evenly spaced notes (or pass empty notes vector)
    pub fn new_with_sequence(period: i32, mut notes: Vec<DynSoundSource>, repeat: u32) -> Self {
        let mut seq = Sequence::new();
        seq.repeat = repeat;
        let mut t_idx: i32 = 0;
        for note in notes.drain(..) {
            seq.add(t_idx, note);
            t_idx += period;
        }
        seq.duration = seq.notes.len() as i32 * period;
        seq
    }

    pub fn set_repeat(&mut self, repeat: u32) {
        self.repeat = repeat;
    }

    pub fn set_duration(&mut self, duration: i32) {
        self.duration = duration;
    }

    fn calculate_duration(&self) -> i32 {
        // self.duration is subtly different to calculated_duration. The first case is the time
        // we use to start repeating and doesn't include any 'ring' time of notes that overlap.
        // This function does account for ring time of whatever notes are playing.
        let mut duration: i32 = 0;
        for note in self.notes.iter() {
            duration = duration.max((*note.sound_source).duration() + note.start_time)
        }
        duration
    }
}

impl SoundSource for Sequence {
    fn next_value(&mut self, n: i32) -> (f32, f32) {
        let mut res1: f32 = 0.0;
        let mut res2: f32 = 0.0;
        let mut time_offset: i32 = 0;
        let mut repeat_count: u32 = 0;
        while n - time_offset >= self.duration {
            time_offset += self.duration;
            repeat_count += 1;
        }
        if repeat_count < self.repeat {
            for note in self.notes.iter_mut() {
                if n - time_offset >= note.start_time
                        && n - time_offset < note.start_time + (*note.sound_source).duration()
                        && !note.is_playing {
                    note.is_playing = true;
                    note.playing_start_time = time_offset + note.start_time;
                }
            }
        }
        for note in self.notes.iter_mut() {
            if note.is_playing {
                if n - note.playing_start_time < (*note.sound_source).duration() {
                    let (v1, v2) = (*note.sound_source).next_value(n - note.playing_start_time);
                    res1 += v1;
                    res2 += v2;
                } else {
                    note.is_playing = false;
                }
            }
        }
        (res1, res2)
    }

    fn duration(&self) -> i32 {
        self.calculate_duration() * self.repeat as i32
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let mut sequence = Sequence::new();
        let repeats = params[0].parse::<u32>().unwrap();
        sequence.set_repeat(repeats);
        let duration = params[1].parse::<f32>().unwrap() * reader.sample_rate as f32;
        for source_def in &params[2..] {
            let parts: Vec<_> = source_def.split(" ").collect();
            let start_time = parts[0].parse::<f32>().unwrap() * reader.sample_rate as f32;
            let source = reader.get_sound(&parts[1]);
            sequence.add(start_time.round() as i32, source);
        }
        if duration > 0.0 {
            sequence.set_duration(duration.round() as i32);
        }
        Box::new(sequence)
    }
}

}