pub mod arpeggios {

use crate::traits::traits::{SoundSource, DynSoundSource, DynInstrument};
use crate::midi_notes::midi_notes::note2freq;
use crate::midi_notes::midi_notes as mn;
use crate::sequence::sequence::Sequence;

pub fn arpeggios(instrument: DynInstrument)  -> DynSoundSource {
    let bpm = 160.0 * 2.0;
    let note_duration = 60.0 / bpm * 0.9;

    // C arpeggio x 3
    let mut vec = Vec::<DynSoundSource>::new();
    vec.push((*instrument).play(note2freq(4, mn::MIDI_OFFSET_C), note_duration, 0.6));
    vec.push((*instrument).play(note2freq(4, mn::MIDI_OFFSET_E), note_duration, 0.4));
    vec.push((*instrument).play(note2freq(4, mn::MIDI_OFFSET_G), note_duration, 0.4));
    vec.push((*instrument).play(note2freq(5, mn::MIDI_OFFSET_C), note_duration, 0.4));
    let sound_source1 = Sequence::new_with_sequence(bpm, vec, 3);

    // C arpeggio with sustain last note
    let mut vec2 = Vec::<DynSoundSource>::new();
    vec2.push((*instrument).play(note2freq(5, mn::MIDI_OFFSET_C), note_duration, 0.6));
    vec2.push((*instrument).play(note2freq(5, mn::MIDI_OFFSET_E), note_duration, 0.4));
    vec2.push((*instrument).play(note2freq(5, mn::MIDI_OFFSET_G), note_duration, 0.4));
    vec2.push((*instrument).play(note2freq(6, mn::MIDI_OFFSET_C), 5.0, 0.7));
    let sound_source2 = Sequence::new_with_sequence(bpm, vec2, 1);

    // concatenate previous two sequences
    let mut sound_source = Sequence::new();
    sound_source.add(0.0, Box::new(sound_source1));
    sound_source.add(sound_source.duration(), Box::new(sound_source2));
    Box::new(sound_source)
}

}
