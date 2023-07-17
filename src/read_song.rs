pub mod read_song {
    use std::fs::File;
    use std::rc::Rc;
    use serde::{Serialize, Deserialize};
    use crate::traits::traits::{DynSoundSource, SoundSource};
    use crate::knob::knob::Knob;
    use crate::midi_notes::midi_notes::{midistr2freq, midi2freq};
    use crate::dc::dc::DC;
    use crate::envelope::envelope::Envelope;
    use crate::multiply::multiply::Multiply;
    use crate::sequence::sequence::Sequence;
    use crate::sine::sine::Sine;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct PatchItem {
        name: String,
        root: String,
        sounds: Vec<SoundItem>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct SoundItem {
        name: String,
        sound_type: String,
        params: Vec<String>
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct YAMLFormat {
        patches: Vec<PatchItem>,
        sounds: Vec<SoundItem>,
        root: String
    }

    struct PatchContextItem {
        params: Vec::<String>,
        patch_index: usize,
    }
    impl PatchContextItem {
        fn from_params(params: &Vec::<String>, patch_index: usize) -> PatchContextItem {
            let mut params_clone = params.clone();
            PatchContextItem { params: params_clone, patch_index: patch_index }
        }
    }

    struct PatchContext {
        stack: Vec<PatchContextItem>,
    }
    impl PatchContext {
        fn new() -> PatchContext {
            PatchContext { stack: Vec::<PatchContextItem>::new() }
        }
        fn push(&mut self, params: &Vec::<String>, patch_index: usize) {
            self.stack.push(PatchContextItem::from_params(params, patch_index));
        }
        fn pop(&mut self) -> Option<PatchContextItem> {
            self.stack.pop()
        }
        fn current(&self) -> &PatchContextItem {
            &self.stack[self.stack.len()-1]
        }
        fn get_param(&self, index: usize) -> String {
            let params = &self.current().params;
            params[index].clone()
        }
        fn active(&self) -> bool {
            self.stack.len() > 0
        }

    }

    pub struct SongReader {
        yaml: YAMLFormat,
        pub sample_rate: i32,
        patch_context: PatchContext
    }

    impl SongReader {

        pub fn get_knob(&mut self, knob_val: &str, dc_scale: f32) -> Knob {
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
                            Err(_) => Knob::new(self.get_sound(knob_val)),
                        }
                    }
                }
            }
        }

        fn get_patch(&mut self, patch_name: &str, params: &Vec::<String>) -> DynSoundSource {
            println!("get_patch({})", patch_name);
            let patch_idx = self.yaml.patches.binary_search_by_key(&patch_name, |s: &PatchItem| &s.name).unwrap();
            let patch_root = self.yaml.patches[patch_idx].root.clone();
            self.patch_context.push(params, patch_idx);
            let res = self.get_sound(&patch_root);
            self.patch_context.pop();
            res
        }

        fn substitute_params(&self, params: &Vec::<String>) -> Vec::<String> {
            let mut new_params = Vec::<String>::new();
            for param in params {
                if param.starts_with("INPUT ") {
                    let substitute_index = param[6..].parse::<usize>().unwrap();
                    new_params.push(self.patch_context.get_param(substitute_index));
                } else {
                    new_params.push(param.clone());
                }
            }
            new_params
        }

        fn get_sound_from_type(&mut self, sound_type: &str, params: &Vec::<String>) -> DynSoundSource {
            let new_params = self.substitute_params(params);
            if sound_type.starts_with("patch ") {
                self.get_patch(&sound_type[6..], &new_params)
            } else {
                match sound_type {
                    "dc" => DC::from_yaml(&new_params, self),
                    "envelope" => Envelope::from_yaml(&new_params, self),
                    "multiply" => Multiply::from_yaml(&new_params, self),
                    "sequence" => Sequence::from_yaml(&new_params, self),
                    "sine" => Sine::from_yaml(&new_params, self),
                    &_ => todo!()
                }
            }
        }

        fn get_patch_sound(&self, sound_name: &str) -> &SoundItem {
            println!("get_patch_sound({})", sound_name);
            let idx = self.patch_context.current().patch_index;
            let patch = &self.yaml.patches[idx];
            let sound_idx = patch.sounds.binary_search_by_key(&sound_name, |s: &SoundItem| &s.name).unwrap();
            &patch.sounds[sound_idx]
        }

        pub fn get_sound(&mut self, sound_name: &str) -> DynSoundSource {
            println!("get_sound({})", sound_name);
            let mut item;
            if self.patch_context.active() {
                item = self.get_patch_sound(sound_name);
            } else {
                let sound_idx = self.yaml.sounds.binary_search_by_key(&sound_name, |s: &SoundItem| &s.name).unwrap();
                item = &self.yaml.sounds[sound_idx];
            }
            self.get_sound_from_type(&item.sound_type.clone(), &item.params.clone())
        }
    }

    pub fn read_song(filename: &str, sample_rate: i32) -> DynSoundSource {
        let f = File::open(filename).unwrap();
        let mut yaml:YAMLFormat = serde_yaml::from_reader(&f).unwrap();
        yaml.sounds.sort_by(|s1: &SoundItem, s2: &SoundItem| s1.name.cmp(&s2.name));
        yaml.patches.sort_by(|s1: &PatchItem, s2: &PatchItem| s1.name.cmp(&s2.name));
        for patch in yaml.patches.iter_mut() {
            patch.sounds.sort_by(|s1: &SoundItem, s2: &SoundItem| s1.name.cmp(&s2.name));
        }
        let mut reader = SongReader {
            yaml: yaml,
            sample_rate: sample_rate,
            patch_context: PatchContext::new() };
        reader.get_sound(&reader.yaml.root.clone())
    }

}
