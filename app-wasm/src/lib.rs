use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/canvas.js")]
extern "C" {
    fn draw(duration: JsValue);
}

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
        console_error_panic_hook::set_once();
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

    pub fn set_density(&mut self, denisity: f32) {
        self.0.set_density(denisity)
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.0.set_volume(volume)
    }

    pub fn set_position(&mut self, position: usize) {
        self.0.set_position(position)
    }

    pub fn set_duration(&mut self, duration: usize) {
        self.0.set_duration(duration)
    }

    pub fn set_new_grain_hook(&mut self) {
        let hook = |duration: usize| unsafe {
            let js: JsValue = duration.into();
            draw(js);
        };
        self.0.set_new_grain_hook(Some(hook));
    }
}
