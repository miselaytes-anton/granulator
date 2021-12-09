use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Granulator(granulator::Granulator);

impl Default for Granulator {
    fn default() -> Self {
        Self(granulator::Granulator::new(41000, 50.0, 3000))
    }
}

#[wasm_bindgen]
impl Granulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process(
        &mut self,
        input_l: &[f32],
        input_r: &[f32],
        output_l: &mut [f32],
        output_r: &mut [f32],
    ) {
        for i in 0..input_l.len() {
            let out = self.0.process([input_l[i] as f32, input_r[i] as f32]);
            output_l[i] = out[0] as f32;
            output_r[i] = out[1] as f32;
        }
    }

    pub fn set_density(&mut self, value: f32) {
        self.0.set_density(value)
    }

    pub fn set_volume(&mut self, value: f32) {
        self.0.set_volume(value)
    }
}
