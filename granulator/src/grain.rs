use crate::constants::{Frame, GRAIN_AMPLITUDE, SILENT_FRAME};

use crate::delay_line::DelayLine;
use crate::parabolic_envelope::ParabolicEnvelope;

#[derive(Copy, Clone)]
pub struct Grain {
    pub is_active: bool,
    duration_samples: f32,
    envelope: ParabolicEnvelope,
    // Position to read from delay line.
    position: f32,
    num_samples_played: f32,

    pitch: f32,
}

impl Grain {
    pub fn new(position: f32, duration_samples: f32, pitch: f32) -> Grain {
        Grain {
            is_active: false,
            duration_samples,
            envelope: ParabolicEnvelope::new(duration_samples as f32, GRAIN_AMPLITUDE),
            position,
            num_samples_played: 0.0,
            pitch,
        }
    }

    pub fn process(&mut self, delay_line: &DelayLine) -> Frame {
        if self.is_active == false {
            return SILENT_FRAME;
        }
        let env = self.envelope.process();

        let [left, right] = delay_line.read(self.position);

        self.num_samples_played += self.pitch;
        self.position += 1.0 - self.pitch;

        if self.num_samples_played >= self.duration_samples {
            self.is_active = false;
            self.num_samples_played = 0.0;
        }

        [left * env, right * env]
    }

    pub fn activate(&mut self, position: f32, duration_samples: f32, pitch: f32) {
        if self.is_active == true {
            return;
        }
        self.position = position;
        self.duration_samples = duration_samples;
        self.num_samples_played = 0.0;
        self.pitch = pitch;
        self.envelope = ParabolicEnvelope::new(duration_samples / self.pitch, GRAIN_AMPLITUDE);
        self.is_active = true;
    }
}
