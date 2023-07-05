pub mod sequence {

use crate::traits::traits::{SoundSource, DynSoundSource};

struct SequenceMember {
    sound_source: DynSoundSource,
    start_time: f32
}

pub struct Sequence {
    notes: Vec<SequenceMember>,
    repeat: u32,
}

impl Sequence {
    pub fn new() -> Self {
        Sequence{ notes: Vec::<SequenceMember>::new(), repeat: 1 }
    }

    // Add notes into the sequence at arbitrary time offsets
    pub fn add(&mut self, start_time: f32, note: DynSoundSource) -> &mut Sequence {
        self.notes.push( SequenceMember { sound_source: note, start_time: start_time } );
        self
    }
    
    // Use new for evenly spaced notes (or pass empty notes vector)
    pub fn new_with_sequence(bpm: f32, mut notes: Vec<DynSoundSource>, repeat: u32) -> Self {
        let mut seq = Sequence::new();
        seq.repeat = repeat;
        let mut t_idx: f32 = 0.0;
        for note in notes.drain(..) {
            seq.add(t_idx, note);
            t_idx += 60.0 / bpm;
        }
        seq
    }

    fn single_duration(&self) -> f32 {
        let mut duration: f32 = 0.0;
        for note in self.notes.iter() {
            duration = duration.max((*note.sound_source).duration() + note.start_time)
        }
        duration
    }
}

impl SoundSource for Sequence {
    fn next_value(&mut self, t: f32) -> (f32, f32) {
        let mut res1: f32 = 0.0;
        let mut res2: f32 = 0.0;
        let duration = self.single_duration();
        let mut time_offset: f32 = 0.0;
        let mut repeat_count: u32 = 0;
        while t - time_offset > duration {
            time_offset += duration;
            repeat_count += 1;
        }
        if repeat_count < self.repeat {
            for note in self.notes.iter_mut() {
                if t - time_offset >= note.start_time {
                    let (v1, v2) = (*note.sound_source).next_value(t - note.start_time - time_offset);
                    res1 += v1;
                    res2 += v2;
                }
            }
        }
        (res1, res2)
    }
    fn duration(&self) -> f32 {
        self.single_duration() * self.repeat as f32
    }
}

}