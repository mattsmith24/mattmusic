pub mod rotation_transfer {

use crate::read_song::read_song::SongReader;
use crate::traits::traits::{SoundSource, DynSoundSource, SoundData};

#[derive(Clone)]
pub struct RotationTransfer {
    input: DynSoundSource,
    angle_of_rotation: f32,
}

impl RotationTransfer {
    pub fn new(input: DynSoundSource, angle_of_rotation: f32,
    ) -> Self {
        RotationTransfer { input: input, angle_of_rotation: angle_of_rotation }
    }
}

struct RotationTransferData {
    input_data: SoundData,
}


impl SoundSource for RotationTransfer {
    fn init_state(&self) -> SoundData {
        Box::new(RotationTransferData {
            input_data: self.input.init_state(),
        })
    }

    fn next_value(&self, n: i32, state: &mut SoundData) -> (f32, f32) {
        let data = &mut state.downcast_mut::<RotationTransferData>().unwrap();
        let c = self.angle_of_rotation.cos();
        let s = self.angle_of_rotation.sin();
        let (x0, x1) = self.input.next_value(n, &mut data.input_data);
        let y0 = c * x0 - s * x1;
        let y1 = s * x0 + c * x1;
        (y0, y1)
    }

    fn duration(&self) -> i32 {
        self.input.duration()
    }

    fn from_yaml(params: &Vec::<String>, reader: &mut SongReader) -> DynSoundSource {
        let input = reader.get_sound(&params[0]);
        let angle_of_rotation = params[1].parse::<f32>().unwrap();
        Box::new(RotationTransfer::new(input, angle_of_rotation))
    }

}

}