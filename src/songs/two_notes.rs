pub mod two_notes {

use crate::traits::traits::{SoundSource, DynSoundSource, DynInstrument};
use crate::midi_notes::midi_notes::note2freq;
use crate::midi_notes::midi_notes as mn;
use crate::sequence::sequence::Sequence;

pub fn TwoNotes(sample_rate: i32, instrument: DynInstrument)  -> DynSoundSource {
    let note_duration = 2 * sample_rate / 4;

    let mut vec = Vec::<DynSoundSource>::new();
    vec.push((*instrument).play(note2freq(4, mn::MIDI_OFFSET_G) / sample_rate as f32, note_duration, 0.5));
    vec.push((*instrument).play(note2freq(3, mn::MIDI_OFFSET_G) / sample_rate as f32, note_duration, 0.5));
    let sound_source = Sequence::new_with_sequence(note_duration, vec, 10);
    Box::new(sound_source)
}

}
