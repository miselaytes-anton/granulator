use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/canvas.js")]
extern "C" {
    fn addGrain(duration: JsValue);
}

#[wasm_bindgen]
pub struct Granulator(granulator::Granulator);

impl Default for Granulator {
    fn default() -> Self {
        Self(granulator::Granulator::new(
            granulator::GranulatorOptions::default(),
        ))
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
            let out = self.0.process((input_l[i] as f32, input_r[i] as f32));
            output_l[i] = out.0 as f32;
            output_r[i] = out.1 as f32;
        }
    }

    pub fn set_density(&mut self, denisity: f32) {
        self.0.set_density(denisity)
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.0.set_volume(volume)
    }

    pub fn set_wet_dry(&mut self, wet_dry: f32) {
        self.0.set_wet_dry(wet_dry)
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.0.set_feedback(feedback)
    }

    pub fn set_position(&mut self, position: f32) {
        self.0.set_position(position)
    }

    pub fn set_duration(&mut self, duration: f32) {
        self.0.set_duration(duration)
    }

    pub fn set_new_grain_hook(&mut self) {
        let hook = |duration: f32| unsafe {
            let js_duration: JsValue = duration.into();
            addGrain(js_duration);
        };
        self.0.set_new_grain_hook(Some(hook));
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.0.set_pitch(pitch)
    }
}
