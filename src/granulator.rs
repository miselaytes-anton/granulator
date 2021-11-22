pub const SAMPLE_RATE: usize = 41000;
const MAX_DELAY_TIME_SECONDS: usize = 10;
const NUM_CHANNELS: usize = 2;

type Frame = [f32; NUM_CHANNELS];

pub struct DelayLine {
    buffer: Vec<Frame>,
    write_index: usize,
}

impl DelayLine {
    pub fn new(max_length: usize) -> Self {
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

pub struct Granulator {
    delay_length: usize,
    delay_line: DelayLine,
}

impl Granulator {
    pub fn new(delay_length: usize) -> Granulator {
        assert!(delay_length <= MAX_DELAY_TIME_SECONDS * SAMPLE_RATE);
        Granulator {
            delay_length: delay_length,
            delay_line: DelayLine::new(MAX_DELAY_TIME_SECONDS * SAMPLE_RATE),
        }
    }
    pub fn process(&mut self, frame: Frame) -> Frame {
        self.delay_line.write_and_advance(frame);

        self.delay_line.read(self.delay_length)
    }
}
