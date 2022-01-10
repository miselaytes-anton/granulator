use crate::constants::{Frame, GRAIN_AMPLITUDE, SILENT_FRAME};

use crate::delay_line::DelayLine;
use crate::parabolic_envelope::ParabolicEnvelope;

#[derive(Copy, Clone)]
pub struct Grain {
    pub is_active: bool,
    duration: f32,
    envelope: ParabolicEnvelope,
    position: usize,
    current_index: f32,
}

impl Grain {
    pub fn new(position: usize, duration: f32) -> Grain {
        Grain {
            is_active: false,
            duration,
            envelope: ParabolicEnvelope::new(duration as f32, GRAIN_AMPLITUDE),
            position,
            current_index: 0.0,
        }
    }

    pub fn process(&mut self, delay_line: &DelayLine) -> Frame {
        if self.is_active == false {
            return SILENT_FRAME;
        }
        let env = self.envelope.process();
        let [left, right] = delay_line.read(self.position as f32);

        self.current_index += 1.0;

        if self.current_index >= self.duration {
            self.is_active = false;
            self.current_index = 0.0;
        }

        [left * env, right * env]
    }

    pub fn activate(&mut self, position: usize, duration: f32) {
        if self.is_active == true {
            return;
        }
        self.position = position;
        self.duration = duration;
        self.envelope = ParabolicEnvelope::new(duration as f32, GRAIN_AMPLITUDE);
        self.is_active = true;
    }
}
