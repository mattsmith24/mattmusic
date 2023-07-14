pub mod many_notes {

use crate::traits::traits::{DynSoundSource, DynInstrument};
use crate::midi_notes::midi_notes::midi2freq;
use crate::sequence::sequence::Sequence;

pub fn many_notes(sample_rate: i32, instrument: DynInstrument)  -> DynSoundSource {
    let note_duration = 1 * sample_rate / 6;

    let mut vec = Vec::<DynSoundSource>::new();
    for n in [58, 63, 70, 72, 65, 60, 67, 62] {
        vec.push((*instrument).play(midi2freq(n) / sample_rate as f32, note_duration * 3/2, 0.5));
    }
    let sound_source = Sequence::new_with_sequence(note_duration, vec, 10);
    Box::new(sound_source)
}

}
