pub mod long_note {
use crate::traits::traits::{DynSoundSource, DynInstrument};
use crate::midi_notes::midi_notes::note2freq;
use crate::midi_notes::midi_notes as mn;

pub fn long_note(instrument: DynInstrument)  -> DynSoundSource {
    (*instrument).play(note2freq(4, mn::MIDI_OFFSET_A), 10.0, 0.5)
}
}