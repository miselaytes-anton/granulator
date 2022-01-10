use crate::constants::Frame;

pub struct DelayLine {
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
