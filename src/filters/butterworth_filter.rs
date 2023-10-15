pub mod butterworth_filter {

use num::complex::Complex;

use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};
use crate::read_song::read_song::SongReader;
use crate::filters::pole_zero_filter::pole_zero_filter::PoleZeroFilter;

#[derive(Clone)]
pub struct ButterworthFilter {
    filter: PoleZeroFilter,
}

fn transform_pole_or_zero(num_points: usize, r: f32) -> Vec<Complex<f32>> {
    let mut res = Vec::<Complex::<f32>>::new();
    println!("r: {}", r);
    // This maths comes from The Theory and Technique of Electronic Music by
    // Miller Puckette and even he says to just take his word for it because the
    // derivation is high level stuff. We should end up with an arc of evenly
    // spaced poles that are inside the unit circle but describing a second
    // intersecting circle that has a center somewhere outside the unit circle
    // on the real axis.
    // The 'r' value is derived from the desired frequency using
    // beta = 2 * arctan(r) where beta is the desired angular frequency
    // (freq * 2 * pi / sample_rate)
    for n in 1..(num_points+1) {
        let alpha = std::f32::consts::PI / 2.0 * ((2.0 * (n as f32) - 1.0) / (num_points as f32) - 1.0);
        println!("alpha: {}", alpha);
        let point = Complex::new((1.0 - r.powf(2.0)) / (1.0 + r.powf(2.0) + 2.0 * r * alpha.cos()),
            (-2.0 * r * alpha.sin()) / (1.0 + r.powf(2.0) + 2.0 * r * alpha.cos()));
        println!("point: {}, {}", point.re, point.im);
        res.push(point);
    }
    res
}

impl ButterworthFilter {
    pub fn new(input: DynSoundSource, num_points: usize, pole_r: f32, zero_r: f32, normalize: f32) -> Self
    {
        let poles = transform_pole_or_zero(num_points, pole_r);
        let zeros = transform_pole_or_zero(num_points, zero_r);
        ButterworthFilter {
            filter: PoleZeroFilter::new(input, poles, zeros, normalize),
        }
    }
}

struct ButterworthFilterData {
    filter_data: SoundData,
}

impl SoundSource for ButterworthFilter {
    fn init_state(&self) -> SoundData {
        Box::new(
            ButterworthFilterData {
                filter_data: self.filter.init_state(),
            }
        )
    }

    fn next_value(&self, n:i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<ButterworthFilterData>().unwrap();
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
        Box::new(ButterworthFilter::new(input, num_points, pole_r, zero_r, normalize))
    }
}

}
