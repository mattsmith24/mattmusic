pub mod read_song {
    use std::fs::File;
    use std::path::Path;
    use serde::{Serialize, Deserialize};
    use crate::traits::traits::{DynSoundSource, SoundSource};
    use crate::knob::knob::Knob;
    use crate::midi_notes::midi_notes::{midistr2freq, midi2freq};
    use crate::dc::dc::DC;
    use crate::envelope::envelope::Envelope;
    use crate::low_pass_filter::low_pass_filter::LowPassFilter;
    use crate::midi2freq::midi2freq::Midi2Freq;
    use crate::mix::mix::Mix;
    use crate::multiply::multiply::Multiply;
    use crate::noise::noise::Noise;
    use crate::pre_render::pre_render::PreRender;
    use crate::sequence::sequence::Sequence;
    use crate::saw::saw::Saw;
    use crate::sine::sine::Sine;
    use crate::square::square::Square;
    use crate::time_box::time_box::TimeBox;
    use crate::triangle::triangle::Triangle;

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
        include: Vec<String>,
        patches: Vec<PatchItem>,
        sounds: Vec<SoundItem>,
        root: String
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct IncludeFormat {
        include: Vec<String>,
        patches: Vec<PatchItem>,
    }

    struct PatchContextItem {
        params: Vec::<String>,
        patch_source_input: String,
        patch_index: usize,
    }

    impl PatchContextItem {
        fn from_params(params: &Vec::<String>, patch_source_input: &str, patch_index: usize) -> PatchContextItem {
            PatchContextItem {
                params: params.clone(),
                patch_source_input: patch_source_input.to_string(),
                patch_index: patch_index }
        }
    }

    struct PatchContext {
        stack: Vec<PatchContextItem>,
        current_idx: i32 // can go negative
    }
    impl PatchContext {
        fn new() -> PatchContext {
            PatchContext { stack: Vec::<PatchContextItem>::new(), current_idx: -1 }
        }
        fn push(&mut self, params: &Vec::<String>, patch_source_input: &str, patch_index: usize) {
            self.stack.push(PatchContextItem::from_params(params, patch_source_input, patch_index));
            self.current_idx = self.stack.len() as i32 - 1;
        }
        fn pop(&mut self) -> Option<PatchContextItem> {
            let res = self.stack.pop();
            self.current_idx = self.stack.len() as i32 - 1;
            res
        }
        fn current(&self) -> &PatchContextItem {
            &self.stack[self.current_idx as usize]
        }
        fn set_parent_context(&mut self) {
            self.current_idx -= 1;
        }
        fn set_child_context(&mut self) {
            self.current_idx += 1;
        }
        fn get_param(&self, index: usize) -> String {
            let params = &self.current().params;
            params[index].clone()
        }
        fn active(&self) -> bool {
            self.current_idx >= 0
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
                let int_parse = knob_val.parse::<i8>();
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

        fn get_patch(&mut self, patch_str: &str, params: &Vec::<String>) -> DynSoundSource {
            println!("get_patch({})", patch_str);
            let parts: Vec<_> = patch_str.split(" ").collect();
            let patch_name = parts[0].clone();
            let patch_source_input;
            if parts.len() > 1 {
                patch_source_input = parts[1].clone();
            } else {
                patch_source_input = "";
            }
            let patch_idx = self.yaml.patches.binary_search_by_key(&patch_name, |s: &PatchItem| &s.name).unwrap();
            let patch_root = self.yaml.patches[patch_idx].root.clone();
            self.patch_context.push(params, patch_source_input, patch_idx);
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
                } else if param.contains(" INPUT ") {
                    let start_pos = param.find(" INPUT ").unwrap();
                    let end_pos: usize;
                    match param[start_pos+7..].find(" ") {
                        Some(p) => end_pos = p + start_pos + 7,
                        None => end_pos = param.len()
                    }
                    let substitute_index = param[start_pos+7..end_pos].parse::<usize>().unwrap();
                    let substitute_param = param[0..start_pos+1].to_string()
                        + &self.patch_context.get_param(substitute_index)
                        + &param[end_pos..];
                    new_params.push(substitute_param);
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
                    "low_pass_filter" => LowPassFilter::from_yaml(&new_params, self),
                    "midi2freq" => Midi2Freq::from_yaml(&new_params, self),
                    "mix" => Mix::from_yaml(&new_params, self),
                    "multiply" => Multiply::from_yaml(&new_params, self),
                    "noise" => Noise::from_yaml(&new_params, self),
                    "pre_render" => PreRender::from_yaml(&new_params, self),
                    "sequence" => Sequence::from_yaml(&new_params, self),
                    "saw" => Saw::from_yaml(&new_params, self),
                    "sine" => Sine::from_yaml(&new_params, self),
                    "square" => Square::from_yaml(&new_params, self),
                    "time_box" => TimeBox::from_yaml(&new_params, self),
                    "triangle" => Triangle::from_yaml(&new_params, self),
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
            if sound_name == "PATCH_INPUT" {
                let patch_source_input = self.patch_context.current().patch_source_input.clone();
                self.patch_context.set_parent_context();
                let res = self.get_sound(&patch_source_input);
                self.patch_context.set_child_context();
                res
            } else {
                let item;
                if self.patch_context.active() {
                    item = self.get_patch_sound(sound_name);
                } else {
                    let sound_idx = self.yaml.sounds.binary_search_by_key(&sound_name, |s: &SoundItem| &s.name).unwrap();
                    item = &self.yaml.sounds[sound_idx];
                }
                self.get_sound_from_type(&item.sound_type.clone(), &item.params.clone())
            }
        }
    }

    fn process_includes(path: &Path, includes: &Vec<String>) -> Vec<PatchItem> {
        let mut res = Vec::<PatchItem>::new();
        for include_fname in includes {
            let full_include_fname = path.join(include_fname);
            let include_file = File::open(full_include_fname).unwrap();
            let mut patch_file: IncludeFormat = serde_yaml::from_reader(&include_file).unwrap();
            let mut sub_patches = process_includes(path, &patch_file.include);
            patch_file.patches.append(&mut sub_patches);
            res.append(&mut patch_file.patches);
        }
        res
    }

    pub fn read_song(filename: &str, sample_rate: i32) -> DynSoundSource {
        let f = File::open(filename).unwrap();
        let mut yaml:YAMLFormat = serde_yaml::from_reader(&f).unwrap();
        // get path of base file then look for include files in that location
        let path = Path::new(filename);
        let parent = path.parent().unwrap();
        let mut patches = process_includes(parent, &yaml.include);
        yaml.patches.append(&mut patches);
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
