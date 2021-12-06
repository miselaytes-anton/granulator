use rand::Rng;

pub const SAMPLE_RATE: usize = 41000;
const MAX_DELAY_TIME_SECONDS: usize = 10;
const NUM_CHANNELS: usize = 2;
const GRAIN_AMPLITUDE: f32 = 0.7;
const MAX_GRAINS: usize = 100;
const DELAY_FEEDBACK: f32 = 0.2;
// 1 - wet, 0 - dry
const WET_DRY: f32 = 1.0;
const OUTPUT_GAIN: f32 = 0.5;

const SILENT_FRAME: Frame = [0.0, 0.0];

type Frame = [f32; NUM_CHANNELS];
struct DelayLine {
    buffer: Vec<Frame>,
    write_index: usize,
}

impl DelayLine {
    pub fn new(max_length: usize, delay_length: usize) -> Self {
        assert!(delay_length <= max_length);

        Self {
            buffer: vec![[0.0, 0.0]; max_length],
            write_index: 0,
        }
    }

    pub fn read(&self, delay_length: usize) -> Frame {
        self.buffer[self.get_read_index(delay_length)]
    }

    pub fn write_and_advance(&mut self, frame: Frame) {
        self.buffer[self.write_index] = frame;

        if self.write_index == self.buffer.len() - 1 {
            self.write_index = 0;
        } else {
            self.write_index += 1;
        }
    }

    fn get_read_index(&self, delay_length: usize) -> usize {
        let read_index = if delay_length > self.write_index {
            self.buffer.len() + self.write_index - delay_length
        } else {
            self.write_index - delay_length
        };
        read_index
    }
}

#[derive(Copy, Clone)]
struct ParabolicEnvelope {
    amplitude: f32,
    slope: f32,
    curve: f32,

    duration_samples: usize,
    grain_amplitude: f32,
}

impl ParabolicEnvelope {
    pub fn new(duration_samples: usize, grain_amplitude: f32) -> ParabolicEnvelope {
        let duration = 1.0 / (duration_samples as f32);
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

#[derive(Copy, Clone)]
struct Grain {
    is_active: bool,
    duration: usize,
    envelope: ParabolicEnvelope,
    position: usize,
    current_index: usize,
}

impl Grain {
    pub fn new(position: usize, duration: usize) -> Grain {
        Grain {
            is_active: false,
            duration,
            envelope: ParabolicEnvelope::new(duration, GRAIN_AMPLITUDE),
            position,
            current_index: 0,
        }
    }

    pub fn process(&mut self, delay_line: &DelayLine) -> Frame {
        if self.is_active == false {
            return SILENT_FRAME;
        }
        let env = self.envelope.process();
        let [left, right] = delay_line.read(self.position);

        self.current_index += 1;

        if self.current_index == self.duration {
            self.is_active = false;
            self.current_index = 0;
        }

        [left * env, right * env]
    }

    pub fn activate(&mut self, position: usize, duration: usize) {
        if self.is_active == true {
            return;
        }
        self.position = position;
        self.duration = duration;
        self.envelope = ParabolicEnvelope::new(duration, GRAIN_AMPLITUDE);
        self.is_active = true;
    }
}

struct Scheduler {
    next_onset: usize,
    grains: [Grain; MAX_GRAINS],
    delay_line: DelayLine,
    position: usize,
    density: f32,
    duration: usize,
}

impl Scheduler {
    pub fn new(delay_line: DelayLine, position: usize, density: f32, duration: usize) -> Scheduler {
        Scheduler {
            next_onset: 0,
            grains: [Grain::new(position, duration); MAX_GRAINS],
            delay_line,
            position,
            density,
            duration,
        }
    }

    pub fn process(&mut self) -> Frame {
        if self.next_onset == 0 {
            self.activate_grain();
            self.next_onset += self.next_interonset_density();
        }
        self.next_onset -= 1;

        self.synthesize_active_grains()
    }

    fn synthesize_active_grains(&mut self) -> Frame {
        let mut num_active_grains: f32 = 0.0;
        let [mut left, mut right]: Frame = SILENT_FRAME;
        let gain: f32 = 2.0;

        for grain in self.grains.iter_mut() {
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

    fn activate_grain(&mut self) {
        for grain in self.grains.iter_mut() {
            if grain.is_active == false {
                grain.activate(self.position, self.duration);
                continue;
            }
        }
    }

    #[allow(dead_code)]
    fn next_interonset_random(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..100)
    }
    #[allow(dead_code)]
    fn next_interonset_density(&self) -> usize {
        let mut rng = rand::thread_rng();
        let random: f32 = rng.gen_range(0.1..1.0);
        let interonset = -(random.ln() / self.density * 1000.0).ceil() as usize;
        if interonset == 0 {
            return 1;
        }
        interonset
    }
    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }
    pub fn set_density(&mut self, density: f32) {
        self.density = density;
    }
    pub fn set_duration(&mut self, duration: usize) {
        self.duration = duration;
    }
}

pub struct Granulator {
    scheduler: Scheduler,
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
            scheduler: Scheduler::new(delay_line, position, density, duration),
        }
    }
    pub fn process(&mut self, frame: Frame) -> Frame {
        let [left, right] = frame;
        let [delayed_left, delayed_right] = self.scheduler.process();

        let dry = 1.0 - WET_DRY;
        let output: Frame = [
            (-left * dry + delayed_left * WET_DRY) * OUTPUT_GAIN,
            (-right * dry + delayed_right * WET_DRY) * OUTPUT_GAIN,
        ];
        let processed_frame: Frame = [
            left + delayed_left * DELAY_FEEDBACK,
            right + delayed_right * DELAY_FEEDBACK,
        ];

        self.scheduler.delay_line.write_and_advance(processed_frame);

        output
    }
    pub fn set_position(&mut self, position: usize) {
        self.scheduler.set_position(position);
    }

    pub fn set_density(&mut self, density: f32) {
        self.scheduler.set_density(density);
    }

    pub fn set_duration(&mut self, duration: usize) {
        self.scheduler.set_duration(duration);
    }
}
