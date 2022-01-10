#[derive(Copy, Clone)]
pub struct ParabolicEnvelope {
    amplitude: f32,
    slope: f32,
    curve: f32,

    duration_samples: f32,
    grain_amplitude: f32,
}

impl ParabolicEnvelope {
    pub fn new(duration_samples: f32, grain_amplitude: f32) -> ParabolicEnvelope {
        let duration = 1.0 / duration_samples;
        let duration2 = duration * duration;
        let slope = 4.0 * grain_amplitude * (duration - duration2);

        ParabolicEnvelope {
            amplitude: 0.0,
            slope,
            curve: -8.0 * grain_amplitude * duration2,

            duration_samples,
            grain_amplitude,
        }
    }

    pub fn process(&mut self) -> f32 {
        self.amplitude = self.amplitude + self.slope;
        self.slope = self.slope + self.curve;

        if self.amplitude < 0.0 {
            let new = ParabolicEnvelope::new(self.duration_samples, self.grain_amplitude);
            self.amplitude = new.amplitude;
            self.slope = new.slope;
            self.curve = new.curve;
        }

        self.amplitude
    }
}
