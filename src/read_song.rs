pub mod read_song {
    use std::fs::File;
    use serde::{Serialize, Deserialize};
    use crate::traits::traits::DynSoundSource;
    use crate::sine::sine::Sine;
    use crate::knob::knob::Knob;
    use crate::sequence::sequence::Sequence;
    use crate::midi_notes::midi_notes::{midistr2freq, midi2freq};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct SoundItem {
        name: String,
        sound_type: String,
        params: Vec<String>
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct YAMLFormat {
        sounds: Vec<SoundItem>,
        root: String
    }

    fn get_knob(items: &Vec<SoundItem>, knob_val: &str, sample_rate: i32, dc_scale: f32) -> Knob {
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
                        Err(_) => Knob::new(get_sound(items, knob_val, sample_rate)),
                    }
                }
            }
        }
    }

    fn get_sound(items: &Vec<SoundItem>, sound_name: &str, sample_rate: i32) -> DynSoundSource {
        println!("get_sound({})", sound_name);
        let sound_idx = items.binary_search_by_key(&sound_name, |s: &SoundItem| &s.name).unwrap();
        let item = &items[sound_idx];
        match item.sound_type.as_str() {
            "sine" => {
                let freq = get_knob(items, &item.params[0], sample_rate, 1.0 / sample_rate as f32);
                let strength = get_knob(items, &item.params[1], sample_rate, 1.0);
                let duration = item.params[2].parse::<f32>().unwrap() * sample_rate as f32;
                Box::new(Sine::new(freq, strength, duration.round() as i32))
            },
            "sequence" => {
                let mut sequence = Sequence::new();
                let repeats = item.params[0].parse::<u32>().unwrap();
                sequence.set_repeat(repeats);
                let duration = item.params[1].parse::<f32>().unwrap() * sample_rate as f32;
                for source_def in &item.params[2..] {
                    let parts: Vec<_> = source_def.split(" ").collect();
                    let start_time = parts[0].parse::<f32>().unwrap() * sample_rate as f32;
                    let source = get_sound(items, &parts[1], sample_rate);
                    sequence.add(start_time.round() as i32, source);
                }
                if duration > 0.0 {
                    sequence.set_duration(duration.round() as i32);
                }
                Box::new(sequence)
            },
            &_ => todo!()
        }
    }

    pub fn read_song(filename: &str, sample_rate: i32) -> DynSoundSource {
        let f = File::open(filename).unwrap();
        let mut yaml:YAMLFormat = serde_yaml::from_reader(&f).unwrap();
        yaml.sounds.sort_by(|s1: &SoundItem, s2: &SoundItem| s1.name.cmp(&s2.name));
        get_sound(&yaml.sounds, &yaml.root, sample_rate)
    }
}
