pub mod read_song {
    use std::fs::File;
    use serde::{Serialize, Deserialize};
    use crate::traits::traits::DynSoundSource;
    use crate::sine::sine::Sine;
    use crate::knob::knob::Knob;

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
        let float_parse = knob_val.parse::<f32>();
        match float_parse {
            Ok(f) => Knob::dc(f * dc_scale),
            Err(_) => Knob::new(get_sound(items, knob_val, sample_rate)),
        }
    }

    fn get_sound(items: &Vec<SoundItem>, sound_name: &str, sample_rate: i32) -> DynSoundSource {
        let sound_idx = items.binary_search_by_key(&sound_name, |s: &SoundItem| &s.name).unwrap();
        let item = &items[sound_idx];
        match item.sound_type.as_str() {
            "sine" => {
                let freq = get_knob(items, &item.params[0], sample_rate, 1.0 / sample_rate as f32);
                let strength = get_knob(items, &item.params[1], sample_rate, 1.0);
                let duration = item.params[2].parse::<f32>().unwrap() * sample_rate as f32;
                Box::new(Sine::new(freq, strength, duration.round() as i32))
            },
            &_ => todo!()
        }
    }

    pub fn read_song(filename: &str, sample_rate: i32) -> DynSoundSource {
        let f = File::open(filename).unwrap();
        let yaml:YAMLFormat = serde_yaml::from_reader(&f).unwrap();
        get_sound(&yaml.sounds, &yaml.root, sample_rate)
    }
}
