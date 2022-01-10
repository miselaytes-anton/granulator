use crate::constants::{
    Frame, DEFAULT_DELAY_FEEDBACK, DEFAULT_VOLUME, DEFAULT_WET_DRY, MAX_DELAY_TIME_SECONDS,
    MAX_GRAINS, SAMPLE_RATE, SILENT_FRAME,
};
use crate::delay_line::DelayLine;
use crate::grain::Grain;
use crate::scheduler::Scheduler;

type NewGrainHook = fn(duration: usize);

pub struct Granulator {
    scheduler: Scheduler,
    grains_pool: [Grain; MAX_GRAINS],
    delay_line: DelayLine,
    position: usize,
    duration: usize,
    volume: f32,
    feedback: f32,
    wet_dry: f32,
    pub new_grain_hook: Option<NewGrainHook>,
}

impl Granulator {
    /**
     * position: 1 - 410000
     * density: 1.0 - 100.0
     * duration: commonly 10 to 70 ms or 400 - 3000 samples for 41000 sr.
     */
    pub fn new(position: usize, density: f32, duration: usize) -> Granulator {
        let delay_line = DelayLine::new(MAX_DELAY_TIME_SECONDS * SAMPLE_RATE, position);
        Granulator {
            scheduler: Scheduler::new(density),
            grains_pool: [Grain::new(position, duration as f32); MAX_GRAINS],
            delay_line,
            position,
            duration,
            volume: DEFAULT_VOLUME,
            feedback: DEFAULT_DELAY_FEEDBACK,
            wet_dry: DEFAULT_WET_DRY,
            new_grain_hook: None,
        }
    }
    pub fn process(&mut self, input_frame: Frame) -> Frame {
        let should_start_new_grain = self.scheduler.advance();
        if should_start_new_grain {
            self.activate_grain();
            match &self.new_grain_hook {
                Some(new_grain_hook) => new_grain_hook(self.duration),
                None => {}
            }
        }

        let synthesized_frame = self.synthesize_active_grains();
        let feedback_frame = self.get_feedback_frame(input_frame, synthesized_frame);

        self.delay_line.write_and_advance(feedback_frame);

        self.get_output_frame(input_frame, synthesized_frame)
    }

    fn get_output_frame(
        &mut self,
        [input_left, input_right]: Frame,
        [synthesized_left, synthesized_right]: Frame,
    ) -> Frame {
        let dry = 1.0 - self.wet_dry;

        [
            (-input_left * dry + synthesized_left * self.wet_dry) * self.volume,
            (-input_right * dry + synthesized_right * self.wet_dry) * self.volume,
        ]
    }

    fn get_feedback_frame(
        &mut self,
        [input_left, input_right]: Frame,
        [synthesized_left, synthesized_right]: Frame,
    ) -> Frame {
        [
            input_left + synthesized_left * self.feedback,
            input_right + synthesized_right * self.feedback,
        ]
    }

    /**
     * Mix output samples of currently active grains.
     */
    fn synthesize_active_grains(&mut self) -> Frame {
        let mut num_active_grains: f32 = 0.0;
        let [mut left, mut right]: Frame = SILENT_FRAME;
        let gain: f32 = 2.0;

        for grain in self.grains_pool.iter_mut() {
            if grain.is_active == true {
                let [left_grain, right_grain] = grain.process(&self.delay_line);
                left += left_grain;
                right += right_grain;
                num_active_grains += 1.0;
            }
        }

        if num_active_grains > 0.0 {
            return [
                left / num_active_grains * gain,
                right / num_active_grains * gain,
            ];
        }

        SILENT_FRAME
    }

    /**
     * Active one grain from the grains pool if available.
     */
    fn activate_grain(&mut self) {
        for grain in self.grains_pool.iter_mut() {
            if grain.is_active == false {
                grain.activate(self.position, self.duration as f32);
                continue;
            }
        }
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    pub fn set_density(&mut self, density: f32) {
        self.scheduler.set_density(density);
    }

    pub fn set_duration(&mut self, duration: usize) {
        self.duration = duration;
    }
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }
    pub fn set_new_grain_hook(&mut self, new_grain_hook: Option<NewGrainHook>) {
        self.new_grain_hook = new_grain_hook;
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback;
    }

    pub fn set_wet_dry(&mut self, wet_dry: f32) {
        self.wet_dry = wet_dry;
    }
}
