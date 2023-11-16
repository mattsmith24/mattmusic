pub mod read_song {
    use std::fs::File;
    use std::sync::{Arc, Mutex};
    use std::path::Path;
    use std::collections::HashMap;
    use serde::{Serialize, Deserialize};
    use evalexpr;

    use crate::traits::traits::{DynSoundSource, SoundSource};
    use crate::knob::knob::{Knob, ComplexKnob};
    use crate::midi_notes::midi_notes::{midistr2freq, midi2freq};

    use crate::buffer_reader::buffer_reader::BufferReader;
    use crate::buffer_writer::buffer_writer::BufferWriter;
    use crate::cauchy_transfer::cauchy_transfer::CauchyTransfer;
    use crate::clip::clip::Clip;
    use crate::cos_transfer::cos_transfer::CosTransfer;
    use crate::db2amp::db2amp::Db2Amp;
    use crate::dc::dc::DC;
    use crate::delay_line::delay_line::DelayLine;
    use crate::envelope::envelope::Envelope;
    use crate::export_wav::export_wav::ExportWav;
    use crate::gaussian_transfer::gaussian_transfer::GaussianTransfer;
    use crate::hann_window::hann_window::HannWindow;
    use crate::import_wav::import_wav::ImportWav;
    use crate::midi2freq::midi2freq::Midi2Freq;
    use crate::mix::mix::Mix;
    use crate::multiply::multiply::Multiply;
    use crate::noise::noise::Noise;
    use crate::oscillator::oscillator::Oscillator;
    use crate::pitch_shift::pitch_shift::PitchShift;
    use crate::pre_render::pre_render::PreRender;
    use crate::pulse_train::pulse_train::PulseTrain;
    use crate::ramp::ramp::Ramp;
    use crate::recirculating_delay::recirculating_delay::RecirculatingDelay;
    use crate::reverberator::reverberator::Reverberator;
    use crate::rotation_transfer::rotation_transfer::RotationTransfer;
    use crate::saw::saw::Saw;
    use crate::sequence::sequence::Sequence;
    use crate::sine::sine::Sine;
    use crate::square::square::Square;
    use crate::time_box::time_box::TimeBox;
    use crate::triangle::triangle::Triangle;
    use crate::uneven_delay::uneven_delay::UnevenDelay;
    use crate::wavetable::wavetable::Wavetable;

    use crate::filters::allpass_filter::allpass_filter::AllpassFilter;
    use crate::filters::butterworth_bandpass_filter::butterworth_bandpass_filter::ButterworthBandpassFilter;
    use crate::filters::butterworth_filter::butterworth_filter::ButterworthFilter;
    use crate::filters::elementary_non_recirculating_filter::elementary_non_recirculating_filter::ElementaryNonRecirculatingFilter;
    use crate::filters::elementary_non_recirculating_filter_2nd_form::elementary_non_recirculating_filter_2nd_form::ElementaryNonRecirculatingFilter2;
    use crate::filters::elementary_recirculating_filter::elementary_recirculating_filter::ElementaryRecirculatingFilter;
    use crate::filters::high_pass_filter::high_pass_filter::HighPassFilter;
    use crate::filters::low_pass_filter::low_pass_filter::LowPassFilter;
    use crate::filters::pole_zero_filter::pole_zero_filter::PoleZeroFilter;
    use crate::filters::real_to_complex::real_to_complex::RealToComplex;

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
        patch_context: PatchContext,
        buffers: HashMap<String,Arc<Mutex<Vec<(f32,f32)>>>>,
    }

    impl SongReader {

        fn parse_knob(&mut self, knob_val: &str, dc_scale: f32) -> DynSoundSource {
            let char1 = knob_val.chars().nth(0).unwrap();
            let note_range = 'A'..'H'; // doesn't include H
            if  note_range.contains(&char1) && knob_val.len() <= 3 {
                Box::new(DC::new(midistr2freq(knob_val) * dc_scale, core::i32::MAX))
            } else {
                let int_parse = knob_val.parse::<i8>();
                match int_parse {
                    Ok(i) => Box::new(DC::new(midi2freq(i) * dc_scale, core::i32::MAX)),
                    Err(_) => {
                        let float_parse = knob_val.parse::<f32>();
                        match float_parse {
                            Ok(f) => Box::new(DC::new(f * dc_scale, core::i32::MAX)),
                            Err(_) => self.get_sound(knob_val),
                        }
                    }
                }
            }
        }

        pub fn get_knob(&mut self, knob_val: &str, dc_scale: f32) -> Knob {
            println!("get_knob({})", knob_val);
            Knob::new(self.parse_knob(knob_val, dc_scale))
        }

        pub fn get_complex_knob(&mut self, knob_val: &str) -> ComplexKnob {
            println!("get_complex_knob({})", knob_val);
            let parts: Vec<_> = knob_val.split(",").collect();
            let magnitude = self.parse_knob(&parts[0], 1.0);
            let angle = self.parse_knob(&parts[1], 1.0);
            ComplexKnob::new(Box::new(RealToComplex::new(magnitude, angle)))
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

        fn substitute_params_in_str(&self, param_str: &str) -> String {
            let substitute_param: String;
            let mut needs_substitution = false;
            let mut start_pos: usize = 0;
            if param_str.starts_with("INPUT(") {
                needs_substitution = true;
            } else if param_str.contains("INPUT(") {
                start_pos = param_str.find("INPUT(").unwrap();
                needs_substitution = true;
            }
            if needs_substitution {
                let end_pos: usize;
                match param_str[start_pos + 6..].find(")") {
                    Some(p) => end_pos = p + start_pos + 6,
                    None => end_pos = param_str.len()
                }
                let substitute_index = param_str[start_pos + 6..end_pos].parse::<usize>().unwrap();
                substitute_param = param_str[0..start_pos].to_string()
                    + &self.patch_context.get_param(substitute_index)
                    + &self.substitute_params_in_str(&param_str[end_pos+1..]);
            } else {
                substitute_param = param_str.clone().to_string();
            }
            substitute_param
        }

        fn substitute_params(&self, params: &Vec::<String>) -> Vec::<String> {
            let mut new_params = Vec::<String>::new();
            for param in params {
                new_params.push(self.substitute_params_in_str(&param));
            }
            new_params
        }

        fn get_const(&self, const_name: &str) -> String {
            match const_name {
                "sample_rate" => format!("{}", self.sample_rate),
                "pi" => format!("{}", std::f32::consts::PI),
                "max_int32" => format!("{}", core::i32::MAX),
                _ => panic!("Unknown const '{}'", const_name)
            }
        }

        fn substitute_const_params_in_str(&self, param_str: &str) -> String {
            let substitute_param: String;
            if param_str.contains("CONST(") {
                let start_pos = param_str.find("CONST(").unwrap();
                let end_pos: usize;
                match param_str[start_pos + 6..].find(")") {
                    Some(p) => end_pos = p + start_pos + 6,
                    None => end_pos = param_str.len()
                }
                let const_name = &param_str[start_pos + 6..end_pos];
                substitute_param = param_str[0..start_pos].to_string()
                    + &self.get_const(const_name)
                    + &self.substitute_const_params_in_str(&param_str[end_pos+1..]);
            } else {
                substitute_param = param_str.clone().to_string();
            }
            substitute_param
        }

        fn substitute_const_params(&self, params: &Vec::<String>) -> Vec::<String> {
            let mut new_params = Vec::<String>::new();
            for param in params {
                new_params.push(self.substitute_const_params_in_str(&param));
            }
            new_params
        }

        fn evaluate_params_in_str(&self, param_str: &str) -> String {
            let evaluated_param: String;
            let mut has_expr = false;
            let mut start_pos: usize = 0;
            if param_str.contains("EXPR(") {
                has_expr = true;
                start_pos = param_str.find("EXPR(").unwrap();
            }
            if has_expr {
                // find end_pos
                let end_pos: usize;
                let mut bracket_count: u32 = 1;
                let mut idx = start_pos + 5; // this should place us at the first char after the (
                let mut chars = param_str.chars(); // iterator on string
                chars.nth(idx-1); // consume chars up to idx
                while bracket_count > 0 {
                    let ch = chars.next().unwrap();
                    if ch == '(' {
                        bracket_count += 1;
                    } else if ch == ')' {
                        bracket_count -= 1;
                    }
                    idx += 1;
                }
                end_pos = idx - 1; // Don't include the last bracket. We also skip the first bracket in the next line
                let context = evalexpr::context_map! {
                    "midi2freq" => Function::new(|argument| {
                        if let Ok(val) = argument.as_int() {
                            Ok(evalexpr::Value::Float(midi2freq(val as i8).into()))
                        } else {
                            Err(evalexpr::EvalexprError::expected_int(argument.clone()))
                        }
                    }),
                }.unwrap(); // Do proper error handling here
                let eval: f32 = evalexpr::eval_float_with_context(&param_str[start_pos + 5..end_pos], &context).unwrap() as f32;
                evaluated_param = param_str[0..start_pos].to_string() // prefix
                    + &eval.to_string() // replace EXPR(blah) with evaluated expression
                    + &self.evaluate_params_in_str(&param_str[end_pos+1..]); // evaluate any other params in the string
            } else {
                evaluated_param = param_str.clone().to_string();
            }
            evaluated_param
        }

        fn evaluate_params(&self, params: &Vec::<String>) -> Vec::<String> {
            let mut new_params = Vec::<String>::new();
            for param in params {
                let p1 = self.evaluate_params_in_str(&param);
                if param.contains("EXPR(") {
                    println!("evaluate_params {} => {}", &param, &p1);
                }
                new_params.push(p1);
            }
            new_params
        }

        fn get_sound_from_type(&mut self, sound_type: &str, params: &Vec::<String>) -> DynSoundSource {
            // Subsitute the INPUT(N) style expressions
            let substituted_params = self.substitute_params(params);
            // Substitute the CONST(blah) style expressions
            let const_substituted_params = self.substitute_const_params(&substituted_params);
            // Substitute the EXPR(maths stuff) style expressions
            let evaluated_params = self.evaluate_params(&const_substituted_params);
            if sound_type.starts_with("patch ") {
                self.get_patch(&sound_type[6..], &evaluated_params)
            } else {
                match sound_type {
                    "allpass_filter" => AllpassFilter::from_yaml(&evaluated_params, self),
                    "buffer_reader" => BufferReader::from_yaml(&evaluated_params, self),
                    "buffer_writer" => BufferWriter::from_yaml(&evaluated_params, self),
                    "butterworth_bandpass_filter" => ButterworthBandpassFilter::from_yaml(&evaluated_params, self),
                    "butterworth_filter" => ButterworthFilter::from_yaml(&evaluated_params, self),
                    "cauchy_transfer" => CauchyTransfer::from_yaml(&evaluated_params, self),
                    "clip" => Clip::from_yaml(&evaluated_params, self),
                    "cos_transfer" => CosTransfer::from_yaml(&evaluated_params, self),
                    "db2amp" => Db2Amp::from_yaml(&evaluated_params, self),
                    "dc" => DC::from_yaml(&evaluated_params, self),
                    "delay_line" => DelayLine::from_yaml(&evaluated_params, self),
                    "elementary_non_recirculating_filter" => ElementaryNonRecirculatingFilter::from_yaml(&evaluated_params, self),
                    "elementary_non_recirculating_filter_2nd_form" => ElementaryNonRecirculatingFilter2::from_yaml(&evaluated_params, self),
                    "elementary_recirculating_filter" => ElementaryRecirculatingFilter::from_yaml(&evaluated_params, self),
                    "envelope" => Envelope::from_yaml(&evaluated_params, self),
                    "export_wav" => ExportWav::from_yaml(&evaluated_params, self),
                    "gaussian_transfer" => GaussianTransfer::from_yaml(&evaluated_params, self),
                    "hann_window" => HannWindow::from_yaml(&evaluated_params, self),
                    "high_pass_filter" => HighPassFilter::from_yaml(&evaluated_params, self),
                    "import_wav" => ImportWav::from_yaml(&evaluated_params, self),
                    "low_pass_filter" => LowPassFilter::from_yaml(&evaluated_params, self),
                    "midi2freq" => Midi2Freq::from_yaml(&evaluated_params, self),
                    "mix" => Mix::from_yaml(&evaluated_params, self),
                    "multiply" => Multiply::from_yaml(&evaluated_params, self),
                    "noise" => Noise::from_yaml(&evaluated_params, self),
                    "oscillator" => Oscillator::from_yaml(&evaluated_params, self),
                    "pitch_shift" => PitchShift::from_yaml(&evaluated_params, self),
                    "pre_render" => PreRender::from_yaml(&evaluated_params, self),
                    "pole_zero_filter" => PoleZeroFilter::from_yaml(&evaluated_params, self),
                    "pulse_train" => PulseTrain::from_yaml(&evaluated_params, self),
                    "ramp" => Ramp::from_yaml(&evaluated_params, self),
                    "recirculating_delay" => RecirculatingDelay::from_yaml(&evaluated_params, self),
                    "reverberator" => Reverberator::from_yaml(&evaluated_params, self),
                    "rotation_transfer" => RotationTransfer::from_yaml(&evaluated_params, self),
                    "sequence" => Sequence::from_yaml(&evaluated_params, self),
                    "saw" => Saw::from_yaml(&evaluated_params, self),
                    "sine" => Sine::from_yaml(&evaluated_params, self),
                    "square" => Square::from_yaml(&evaluated_params, self),
                    "time_box" => TimeBox::from_yaml(&evaluated_params, self),
                    "triangle" => Triangle::from_yaml(&evaluated_params, self),
                    "uneven_delay" => UnevenDelay::from_yaml(&evaluated_params, self),
                    "wavetable" => Wavetable::from_yaml(&evaluated_params, self),
                    &_ => todo!("sound_type: {}", sound_type)
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

        pub fn get_buffer(&mut self, buffer_name: &str) -> Arc<Mutex<Vec<(f32,f32)>>> {
            println!("get_buffer({})", buffer_name);
            if let Some(buf) = self.buffers.get(buffer_name) {
                buf.clone()
            } else {
                let buf = Arc::new(Mutex::new(Vec::<(f32,f32)>::new()));
                self.buffers.insert(buffer_name.to_string(), buf.clone());
                buf
            }
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
                    if let Ok(sound_idx) = self.yaml.sounds.binary_search_by_key(&sound_name, |s: &SoundItem| &s.name) {
                        item = &self.yaml.sounds[sound_idx];
                    } else {
                        panic!("get_sound: Couldn't find {}", sound_name);
                    }
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
            patch_context: PatchContext::new(),
            buffers: HashMap::<String, Arc<Mutex<Vec<(f32,f32)>>>>::new(),
        };
        reader.get_sound(&reader.yaml.root.clone())
    }

}
