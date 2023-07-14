pub mod arpeggios {

use crate::traits::traits::{DynSoundSource, DynInstrument};
use crate::midi_notes::midi_notes::note2freq;
use crate::midi_notes::midi_notes as mn;
use crate::sequence::sequence::Sequence;

pub fn arpeggios(sample_rate: i32, instrument: DynInstrument)  -> DynSoundSource {
    let bpm: f32 = 160.0 * 2.0;
    let period = (60.0 / bpm * sample_rate as f32).round() as i32;
    let note_duration = (60.0 / bpm * 1.0 * sample_rate as f32).round() as i32;

    // C arpeggio x 3
    let mut vec = Vec::<DynSoundSource>::new();
    vec.push((*instrument).play(note2freq(4, mn::MIDI_OFFSET_C) / sample_rate as f32, note_duration, 0.6));
    vec.push((*instrument).play(note2freq(4, mn::MIDI_OFFSET_E) / sample_rate as f32, note_duration, 0.4));
    vec.push((*instrument).play(note2freq(4, mn::MIDI_OFFSET_G) / sample_rate as f32, note_duration, 0.4));
    vec.push((*instrument).play(note2freq(5, mn::MIDI_OFFSET_C) / sample_rate as f32, note_duration, 0.4));
    let sound_source1 = Sequence::new_with_sequence(period, vec, 3);

    // C arpeggio with sustain last note
    let mut vec2 = Vec::<DynSoundSource>::new();
    vec2.push((*instrument).play(note2freq(5, mn::MIDI_OFFSET_C) / sample_rate as f32, note_duration, 0.6));
    vec2.push((*instrument).play(note2freq(5, mn::MIDI_OFFSET_E) / sample_rate as f32, note_duration, 0.4));
    vec2.push((*instrument).play(note2freq(5, mn::MIDI_OFFSET_G) / sample_rate as f32, note_duration, 0.4));
    vec2.push((*instrument).play(note2freq(6, mn::MIDI_OFFSET_C) / sample_rate as f32, 5 * sample_rate, 0.7));
    let sound_source2 = Sequence::new_with_sequence(period, vec2, 1);

    // concatenate previous two sequences
    let mut sound_source = Sequence::new();
    sound_source.add(0, Box::new(sound_source1));
    sound_source.add(period * 3 * 4, Box::new(sound_source2));
    Box::new(sound_source)
}

}
