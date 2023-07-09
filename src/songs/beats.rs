pub mod beats {
    use crate::traits::traits::{DynSoundSource, DynInstrument};
    use crate::midi_notes::midi_notes::note2freq;
    use crate::midi_notes::midi_notes as mn;
    use crate::sequence::sequence::Sequence;

    pub fn beats(sample_rate: i32, instrument: DynInstrument)  -> DynSoundSource {
        let bpm: f32 = 120.0 / 2.0;
        let note_duration = (60.0 / bpm * 44000.0).round() as i32;
        let mut vec = Vec::<DynSoundSource>::new();
        vec.push((*instrument).play(note2freq(4, mn::MIDI_OFFSET_A) / sample_rate as f32, note_duration, 0.5));
        let sound_source = Sequence::new_with_sequence(note_duration, vec, 64);
        Box::new(sound_source)
    }
}
