pub mod arpeggios {

use crate::sound_source::sound_source::{SoundSource, DynSoundSource};
use crate::pure_tone::pure_tone::PureTone;
use crate::midi_notes::midi_notes::note2freq;
use crate::midi_notes::midi_notes as mn;
use crate::tremolo::tremolo::Tremolo;
use crate::ding_envelope::ding_envelope::DingEnvelope;
use crate::sequence::sequence::Sequence;


fn vibraphone(octave: u8, pitch: u8, duration: f32) -> DynSoundSource {
    Box::new(
    Tremolo::new(5.0, 0.5, Box::new(
        DingEnvelope::new(2.0, duration, Box::new(
            PureTone::new(note2freq(octave, pitch), 0.5, duration * 2.0)
        ))
    )))
}

pub fn arpeggios()  -> DynSoundSource {
    let bpm = 160.0 * 2.0;
    let note_duration = 60.0 / bpm * 0.9;

    // C arpeggio x 3
    let mut vec = Vec::<DynSoundSource>::new();
    vec.push(vibraphone(4, mn::MIDI_OFFSET_C, note_duration));
    vec.push(vibraphone(4, mn::MIDI_OFFSET_E, note_duration));
    vec.push(vibraphone(4, mn::MIDI_OFFSET_G, note_duration));
    vec.push(vibraphone(5, mn::MIDI_OFFSET_C, note_duration));
    let sound_source1 = Sequence::new_with_sequence(bpm, vec, 3);

    // C arpeggio with sustain last note
    let mut vec2 = Vec::<DynSoundSource>::new();
    vec2.push(vibraphone(5, mn::MIDI_OFFSET_C, note_duration));
    vec2.push(vibraphone(5, mn::MIDI_OFFSET_E, note_duration));
    vec2.push(vibraphone(5, mn::MIDI_OFFSET_G, note_duration));
    vec2.push(vibraphone(6, mn::MIDI_OFFSET_C, 5.0));
    let sound_source2 = Sequence::new_with_sequence(bpm, vec2, 1);

    // concatenate previous two sequences
    let mut sound_source = Sequence::new();
    sound_source.add(0.0, Box::new(sound_source1));
    sound_source.add(sound_source.duration(), Box::new(sound_source2));
    Box::new(sound_source)
}

}
