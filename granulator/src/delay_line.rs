use crate::frame::Frame;

// 10 seconds with 41000 sample rate
const MAX_DELAY_LENGTH_SAMPLES: usize = 410000;

pub struct DelayLine {
    buffer: [Frame; MAX_DELAY_LENGTH_SAMPLES],
    write_index: usize,
    pub max_length: f32,
}

impl DelayLine {
    pub fn new() -> Self {
        Self {
            buffer: [(0.0, 0.0); MAX_DELAY_LENGTH_SAMPLES],
            write_index: 0,
            max_length: MAX_DELAY_LENGTH_SAMPLES as f32,
        }
    }

    /**
     * Get interpolated value from buffer.
     */
    pub fn read(&self, delay_length: f32) -> Frame {
        let index_fractional = self.get_read_index_fractional(delay_length);
        let index_next = index_fractional.ceil();
        let index_next = if index_next >= self.max_length {
            self.max_length - 1.0
        } else {
            index_next
        };
        let index_previous = if index_next == 0.0 {
            self.max_length - 1.0
        } else {
            index_next - 1.0
        };
        let delta = (index_next - index_fractional).abs();

        let (previous_left, previous_right) = self.buffer[index_previous as usize];
        let (next_left, next_right) = self.buffer[index_next as usize];

        let result = (
            next_left + delta * (previous_left - next_left),
            next_right + delta * (previous_right - next_right),
        );
        result
    }

    pub fn write_and_advance(&mut self, frame: Frame) {
        self.buffer[self.write_index] = frame;

        if self.write_index == self.buffer.len() - 1 {
            self.write_index = 0;
        } else {
            self.write_index += 1;
        }
    }

    /**
     * Read index = write index - delay length.
     * Can be in then range [0, max_length - 1]
     */
    fn get_read_index_fractional(&self, mut delay_length: f32) -> f32 {
        if delay_length < 0.0 {
            // Trying to read the future.
            delay_length = 0.0;
        }
        if delay_length >= self.max_length {
            // Going to far in the past.
            delay_length = self.max_length - 1.0;
        }

        let write_index_f32 = self.write_index as f32;
        let read_index = if delay_length > write_index_f32 {
            self.max_length - 1.0 + write_index_f32 - delay_length
        } else {
            write_index_f32 - delay_length
        };
        read_index
    }
}
