pub mod butterworth_bandpass_filter {

use num::complex::Complex;

use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};
use crate::read_song::read_song::SongReader;
use crate::knob::knob::ComplexKnob;
use crate::dc::dc::DC;
use crate::filters::butterworth_filter::butterworth_filter::transform_pole_or_zero;
use crate::filters::pole_zero_filter::pole_zero_filter::PoleZeroFilter;

#[derive(Clone)]
pub struct ButterworthBandpassFilter {
    filter: PoleZeroFilter,
}

fn transform_bandpass_pole_or_zero(a:f32, b:f32, point:&Complex<f32>) -> Vec<Complex<f32>> {
    // This maths comes from The Theory and Technique of Electronic Music by
    // Miller Puckette
    let mut res = Vec::<Complex::<f32>>::new();
    res.push(-(a*(-point).sqrt() - b) / (-b*(-point).sqrt() + a));
    res.push(-(-a*(-point).sqrt() - b) / (b*(-point).sqrt() + a));
    for p in &res {
        println!("bandpass point: {}", p);
    }
    res
}

impl ButterworthBandpassFilter {
    pub fn new(input: DynSoundSource,
        num_points: usize,
        pole_r: f32,
        zero_r: f32,
        center_freq: f32,
        normalize: f32) -> Self
    {
        println!("center_freq: {}", center_freq);
        let low_pass_poles = transform_pole_or_zero(num_points, pole_r);
        let low_pass_zeros = transform_pole_or_zero(num_points, zero_r);
        let π = std::f32::consts::PI;
        let ω = center_freq;
        let a = ( π/4.0 - ω/2.0 ).cos();
        let b = ( π/4.0 - ω/2.0 ).sin();
        println!("a: {}, b: {}", a, b);
        let mut poles = Vec::<Complex::<f32>>::new();
        let mut zeros = Vec::<Complex::<f32>>::new();
        for pole in &low_pass_poles {
            poles.append(&mut transform_bandpass_pole_or_zero(a, b, pole));
        }
        for zero in &low_pass_zeros {
            zeros.append(&mut transform_bandpass_pole_or_zero(a, b, zero));
        }
        // Create dc knobs
        let mut pole_knobs = Vec::<ComplexKnob>::new();
        for pole in &poles {
            pole_knobs.push(ComplexKnob::dc(pole.clone()));
        }
        let mut zero_knobs = Vec::<ComplexKnob>::new();
        for zero in &zeros {
            zero_knobs.push(ComplexKnob::dc(zero.clone()));
        }
        let duration = input.duration();
        let normalize_dc = Box::new(DC::new(normalize, duration));
        ButterworthBandpassFilter {
            filter: PoleZeroFilter::new(input, normalize_dc, pole_knobs, zero_knobs),
        }
    }
}

struct ButterworthBandpassFilterData {
    filter_data: SoundData,
}

impl SoundSource for ButterworthBandpassFilter {
    fn init_state(&self) -> SoundData {
        Box::new(
            ButterworthBandpassFilterData {
                filter_data: self.filter.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<ButterworthBandpassFilterData>().unwrap();
        self.filter.next_value(n, &mut data.filter_data)
    }

    fn duration(&self) -> i32 {
        self.filter.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let normalize = params[1].parse::<f32>().unwrap();
        let num_points = params[2].parse::<usize>().unwrap();
        let pole_r = params[3].parse::<f32>().unwrap();
        let zero_r = params[4].parse::<f32>().unwrap();
        // Convert Hz to angular freq
        let center_freq = params[5].parse::<f32>().unwrap() / (reader.sample_rate as f32) * 2.0 * std::f32::consts::PI;
        Box::new(ButterworthBandpassFilter::new(input, num_points, pole_r, zero_r, center_freq, normalize))
    }
}


}
