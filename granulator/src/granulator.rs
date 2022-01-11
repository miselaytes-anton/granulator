use crate::delay_line::DelayLine;
use crate::frame::{Frame, SILENT_FRAME};
use crate::grain::Grain;
use crate::scheduler::Scheduler;

type NewGrainHook = fn(duration: usize);

const DEFAULT_SAMPLE_RATE: usize = 41000;
const MAX_DELAY_TIME_SECONDS: usize = 10;
const MAX_GRAINS: usize = 100;

type Density = f32;
type Position = usize;
type Duration = usize;
type Pitch = f32;
type Volume = f32;
type Feedback = f32;
type WetDry = f32;

pub struct Granulator {
    scheduler: Scheduler,
    grains_pool: [Grain; MAX_GRAINS],
    delay_line: DelayLine,
    position: Position,
    duration: Duration,
    pitch: Pitch,
    volume: Volume,
    feedback: Feedback,
    wet_dry: WetDry,
    pub new_grain_hook: Option<NewGrainHook>,
}

pub struct GranulatorOptions {
    // 1 - 410000
    pub position: Position,
    // 1.0 - 100.0
    pub density: Density,
    // in samples, commonly 400 - 3000 samples (so that it matches 10 to 70 ms for 41000 sr)
    pub duration: Duration,
    // 0.1 - 10.0
    pub pitch: Pitch,
    pub volume: Volume,
    pub feedback: Feedback,
    pub wet_dry: WetDry,
    pub new_grain_hook: Option<NewGrainHook>,
}

impl Default for GranulatorOptions {
    fn default() -> Self {
        GranulatorOptions {
            position: DEFAULT_SAMPLE_RATE,
            density: 50.0,
            duration: 3000,
            pitch: 1.0,
            volume: 0.5,
            feedback: 0.6,
            wet_dry: 1.0,
            new_grain_hook: None,
        }
    }
}

impl Granulator {
    pub fn new(options: GranulatorOptions) -> Granulator {
        let position = options.position;
        let duration = options.duration;
        let density = options.density;
        let pitch = options.pitch;
        let volume = options.volume;
        let feedback = options.feedback;
        let wet_dry = options.wet_dry;
        let new_grain_hook = options.new_grain_hook;
        let delay_line = DelayLine::new(MAX_DELAY_TIME_SECONDS * DEFAULT_SAMPLE_RATE, position);

        Granulator {
            scheduler: Scheduler::new(density),
            grains_pool: [Grain::new(position as f32, duration as f32, pitch); MAX_GRAINS],
            delay_line,
            position,
            duration,
            pitch,
            volume,
            feedback,
            wet_dry,
            new_grain_hook,
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
                grain.activate(self.position as f32, self.duration as f32, self.pitch);
                continue;
            }
        }
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn set_density(&mut self, density: Density) {
        self.scheduler.set_density(density);
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }
    pub fn set_volume(&mut self, volume: Volume) {
        self.volume = volume;
    }
    pub fn set_new_grain_hook(&mut self, new_grain_hook: Option<NewGrainHook>) {
        self.new_grain_hook = new_grain_hook;
    }

    pub fn set_feedback(&mut self, feedback: Feedback) {
        self.feedback = feedback;
    }

    pub fn set_wet_dry(&mut self, wet_dry: WetDry) {
        self.wet_dry = wet_dry;
    }

    pub fn set_pitch(&mut self, pitch: Pitch) {
        self.pitch = pitch;
    }
}
