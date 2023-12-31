pub mod midi_notes {

const MIDI_NOTES : [(u8, &'static str, u8, f32, f32); 99] = [
    (21,"A",0,27.5,27.00),
    (22,"A#",0,29.1353,28.61),
    (23,"B",0,30.8677,30.31),
    (24,"C",1,32.7032,32.11),
    (25,"C#",1,34.6479,34.02),
    (26,"D",1,36.7081,36.04),
    (27,"D#",1,38.8909,38.18),
    (28,"E",1,41.2035,40.45),
    (29,"F",1,43.6536,42.86),
    (30,"F#",1,46.2493,45.41),
    (31,"G",1,48.9995,48.11),
    (32,"G#",1,51.913,50.97),
    (33,"A",1,55.0,54.00),
    (34,"A#",1,58.2705,57.21),
    (35,"B",1,61.7354,60.61),
    (36,"C",2,65.4064,64.22),
    (37,"C#",2,69.2957,68.04),
    (38,"D",2,73.4162,72.08),
    (39,"D#",2,77.7817,76.37),
    (40,"E",2,82.4069,80.91),
    (41,"F",2,87.3071,85.72),
    (42,"F#",2,92.4986,90.82),
    (43,"G",2,97.9989,96.22),
    (44,"G#",2,103.826,101.94),
    (45,"A",2,110.0,108.00),
    (46,"A#",2,116.541,114.42),
    (47,"B",2,123.471,121.23),
    (48,"C",3,130.813,128.43),
    (49,"C#",3,138.591,136.07),
    (50,"D",3,146.832,144.16),
    (51,"D#",3,155.563,152.74),
    (52,"E",3,164.814,161.82),
    (53,"F",3,174.614,171.44),
    (54,"F#",3,184.997,181.63),
    (55,"G",3,195.998,192.43),
    (56,"G#",3,207.652,203.88),
    (57,"A",3,220.0,216.00),
    (58,"A#",3,233.082,228.84),
    (59,"B",3,246.942,242.45),
    (60,"C",4,261.626,256.87),
    (61,"C#",4,277.183,272.14),
    (62,"D",4,293.665,288.33),
    (63,"D#",4,311.127,305.47),
    (64,"E",4,329.628,323.63),
    (65,"F",4,349.228,342.88),
    (66,"F#",4,369.994,363.27),
    (67,"G",4,391.995,384.87),
    (68,"G#",4,415.305,407.75),
    (69,"A",4,440.0,432.00),
    (70,"A#",4,466.164,457.69),
    (71,"B",4,493.883,484.90),
    (72,"C",5,523.251,513.74),
    (73,"C#",5,554.365,544.29),
    (74,"D",5,587.33,576.65),
    (75,"D#",5,622.254,610.94),
    (76,"E",5,659.255,647.27),
    (77,"F",5,698.456,685.76),
    (78,"F#",5,739.989,726.53),
    (79,"G",5,783.991,769.74),
    (80,"G#",5,830.609,815.51),
    (81,"A",5,880.0,864.00),
    (82,"A#",5,932.328,915.38),
    (83,"B",5,987.767,969.81),
    (84,"C",6,1046.5,1027.47),
    (85,"C#",6,1108.73,1088.57),
    (86,"D",6,1174.66,1153.30),
    (87,"D#",6,1244.51,1221.88),
    (88,"E",6,1318.51,1294.54),
    (89,"F",6,1396.91,1371.51),
    (90,"F#",6,1479.98,1453.07),
    (91,"G",6,1567.98,1539.47),
    (92,"G#",6,1661.22,1631.01),
    (93,"A",6,1760.0,1728.00),
    (94,"A#",6,1864.66,1830.75),
    (95,"B",6,1975.53,1939.61),
    (96,"C",7,2093.0,2054.95),
    (97,"C#",7,2217.46,2177.14),
    (98,"D",7,2349.32,2306.60),
    (99,"D#",7,2489.02,2443.76),
    (100,"E",7,2637.02,2589.07),
    (101,"F",7,2793.83,2743.03),
    (102,"F#",7,2959.96,2906.14),
    (103,"G",7,3135.96,3078.95),
    (104,"G#",7,3322.44,3262.03),
    (105,"A",7,3520.0,3456.00),
    (106,"A#",7,3729.31,3661.50),
    (107,"B",7,3951.07,3879.23),
    (108,"C",8,4186.01,4109.90),
    (109,"C#",8,4434.92,4354.29),
    (110,"D",8,4698.63,4613.21),
    (111,"D#",8,4978.03,4887.52),
    (112,"E",8,5274.04,5178.15),
    (113,"F",8,5587.65,5486.06),
    (114,"F#",8,5919.91,5812.28),
    (115,"G",8,6271.93,6157.89),
    (116,"G#",8,6644.88,6524.06),
    (117,"A",8,7040.0,6912.00),
    (118,"A#",8,7458.62,7323.01),
    (119,"B",8,7902.13,7758.46),
];

#[allow(dead_code)] pub const MIDI_OFFSET_A: u8 = 9u8;
#[allow(dead_code)] pub const MIDI_OFFSET_A_SHARP: u8 = 10u8;
#[allow(dead_code)] pub const MIDI_OFFSET_B_FLAT: u8 = 10u8;
#[allow(dead_code)] pub const MIDI_OFFSET_B: u8 = 11u8;
#[allow(dead_code)] pub const MIDI_OFFSET_C: u8 = 0u8;
#[allow(dead_code)] pub const MIDI_OFFSET_C_SHARP: u8 = 1u8;
#[allow(dead_code)] pub const MIDI_OFFSET_D_FLAT: u8 = 1u8;
#[allow(dead_code)] pub const MIDI_OFFSET_D: u8 = 2u8;
#[allow(dead_code)] pub const MIDI_OFFSET_D_SHARP: u8 = 3u8;
#[allow(dead_code)] pub const MIDI_OFFSET_E_FLAT: u8 = 3u8;
#[allow(dead_code)] pub const MIDI_OFFSET_E: u8 = 4u8;
#[allow(dead_code)] pub const MIDI_OFFSET_F: u8 = 5u8;
#[allow(dead_code)] pub const MIDI_OFFSET_F_SHARP: u8 = 6u8;
#[allow(dead_code)] pub const MIDI_OFFSET_G_FLAT: u8 = 6u8;
#[allow(dead_code)] pub const MIDI_OFFSET_G: u8 = 7u8;
#[allow(dead_code)] pub const MIDI_OFFSET_G_SHARP: u8 = 8u8;
#[allow(dead_code)] pub const MIDI_OFFSET_A_FLAT: u8 = 8u8;

const MIDI_OFFSET_OCTAVE_1: u8 = 3u8;

fn midi_octave_offset(oct: u8) -> i8 {
    MIDI_OFFSET_OCTAVE_1 as i8 - 12 + oct as i8 * 12
}

pub fn note2freq(octave: u8, pitch: u8) -> f32 {
    let offset: i8 = midi_octave_offset(octave) + pitch as i8;
    if offset < 0 {
        panic!("Note too low {} {}", octave, pitch)
    }
    MIDI_NOTES[offset as usize].3
}

pub fn midi2freq(midi: i8) -> f32 {
    440.0 * 2.0_f32.powf((midi - 69) as f32 / 12.0)
}

pub fn midistr2freq(midi: &str) -> f32 {
    if midi.len() > 3 {
        panic!("Couldn't parse \"{}\" as a midi note", midi);
    }
    let note_name = &midi[0..midi.len()-1];
    let octave = midi[midi.len()-1..].parse::<u8>().unwrap();
    match note_name {
        "A" => note2freq(octave, MIDI_OFFSET_A),
        "A#" => note2freq(octave, MIDI_OFFSET_A_SHARP),
        "Bb" => note2freq(octave, MIDI_OFFSET_B_FLAT),
        "B" => note2freq(octave, MIDI_OFFSET_B),
        "C" => note2freq(octave, MIDI_OFFSET_C),
        "C#" => note2freq(octave, MIDI_OFFSET_C_SHARP),
        "Db" => note2freq(octave, MIDI_OFFSET_D_FLAT),
        "D" => note2freq(octave, MIDI_OFFSET_D),
        "D#" => note2freq(octave, MIDI_OFFSET_D_SHARP),
        "Eb" => note2freq(octave, MIDI_OFFSET_E_FLAT),
        "E" => note2freq(octave, MIDI_OFFSET_E),
        "F" => note2freq(octave, MIDI_OFFSET_F),
        "F#" => note2freq(octave, MIDI_OFFSET_F_SHARP),
        "Gb" => note2freq(octave, MIDI_OFFSET_G_FLAT),
        "G" => note2freq(octave, MIDI_OFFSET_G),
        "G#" => note2freq(octave, MIDI_OFFSET_G_SHARP),
        "Ab" => note2freq(octave, MIDI_OFFSET_A_FLAT),
        _ => todo!()
    }
}

}