pub mod read_song {
    use std::fs::File;
    use serde::{Serialize, Deserialize};
    use crate::traits::traits::{DynSoundSource, SoundSource};
    use crate::knob::knob::Knob;
    use crate::midi_notes::midi_notes::{midistr2freq, midi2freq};
    use crate::dc::dc::DC;
    use crate::envelope::envelope::Envelope;
    use crate::sequence::sequence::Sequence;
    use crate::sine::sine::Sine;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct SoundItem {
        name: String,
        sound_type: String,
        params: Vec<String>
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct YAMLFormat {
        patches:Vec<SoundItem>,
        sounds: Vec<SoundItem>,
        root: String
    }

    pub fn get_knob(knob_val: &str, dc_scale: f32, yaml: &YAMLFormat, sample_rate: i32) -> Knob {
        println!("get_knob({})", knob_val);
        let char1 = knob_val.chars().nth(0).unwrap();
        let note_range = 'A'..'H'; // doesn't include H
        if  note_range.contains(&char1) && knob_val.len() <= 3 {
            Knob::dc(midistr2freq(knob_val) * dc_scale)
        } else {
            let int_parse = knob_val.parse::<u8>();
            match int_parse {
                Ok(i) => Knob::dc(midi2freq(i) * dc_scale),
                Err(_) => {
                    let float_parse = knob_val.parse::<f32>();
                    match float_parse {
                        Ok(f) => Knob::dc(f * dc_scale),
                        Err(_) => Knob::new(get_sound(knob_val, yaml, sample_rate)),
                    }
                }
            }
        }
    }

    fn get_patch(patch_name: &str, params: &Vec::<String>, yaml: &YAMLFormat, sample_rate: i32) -> DynSoundSource {
        println!("get_patch({})", patch_name);
        let patch_idx = yaml.patches.binary_search_by_key(&patch_name, |s: &SoundItem| &s.name).unwrap();
        let patch = &yaml.patches[patch_idx];
        let mut new_params = Vec::<String>::new();
        for p in &patch.params {
            if p.starts_with("INPUT ") {
                let idx = p[6..].parse::<usize>().unwrap();
                new_params.push(params[idx].clone());
            } else {
                new_params.push(p.clone());
            }
        }
        get_sound_from_type(&patch.sound_type, &new_params, yaml, sample_rate)
    }

    fn get_sound_from_type(sound_type: &str, params: &Vec::<String>, yaml: &YAMLFormat, sample_rate: i32) -> DynSoundSource {
        if sound_type.starts_with("patch ") {
            get_patch(&sound_type[6..], &params, yaml, sample_rate)
        } else {
            match sound_type {
                "dc" => DC::from_yaml(params, yaml, sample_rate),
                "envelope" => Envelope::from_yaml(params, yaml, sample_rate),
                "sequence" => Sequence::from_yaml(params, yaml, sample_rate),
                "sine" => Sine::from_yaml(params, yaml, sample_rate),
                &_ => todo!()
            }
        }
    }

    pub fn get_sound(sound_name: &str, yaml: &YAMLFormat, sample_rate: i32) -> DynSoundSource {
        println!("get_sound({})", sound_name);
        let sound_idx = yaml.sounds.binary_search_by_key(&sound_name, |s: &SoundItem| &s.name).unwrap();
        let item = &yaml.sounds[sound_idx];
        get_sound_from_type(&item.sound_type, &item.params, yaml, sample_rate)
    }

    pub fn read_song(filename: &str, sample_rate: i32) -> DynSoundSource {
        let f = File::open(filename).unwrap();
        let mut yaml:YAMLFormat = serde_yaml::from_reader(&f).unwrap();
        yaml.sounds.sort_by(|s1: &SoundItem, s2: &SoundItem| s1.name.cmp(&s2.name));
        yaml.patches.sort_by(|s1: &SoundItem, s2: &SoundItem| s1.name.cmp(&s2.name));
        get_sound(&yaml.root, &yaml, sample_rate)
    }
}
