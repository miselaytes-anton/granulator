use crate::constants::Frame;

pub struct DelayLine {
    buffer: Vec<Frame>,
    write_index: usize,
    max_length: f32,
}

impl DelayLine {
    pub fn new(max_length: usize, delay_length: usize) -> Self {
        assert!(delay_length <= max_length);

        Self {
            buffer: vec![[0.0, 0.0]; max_length],
            write_index: 0,
            max_length: max_length as f32,
        }
    }

    /**
     * Get interpolated value from buffer.
     */
    pub fn read(&self, delay_length: f32) -> Frame {
        let index_fractional = self.get_read_index_fractional(delay_length);
        let index_next = index_fractional.ceil();
        let index_previous = if index_next == 0.0 {
            self.max_length - 1.0
        } else {
            index_next - 1.0
        };
        let delta = index_next - index_fractional;

        let [previous_left, previous_right] = self.buffer[index_previous as usize];
        let [next_left, next_right] = self.buffer[index_next as usize];

        let result = [
            next_left + delta * (previous_left - next_left),
            next_right + delta * (previous_right - next_right),
        ];
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

    fn get_read_index_fractional(&self, delay_length: f32) -> f32 {
        let write_index_f32 = self.write_index as f32;
        let read_index = if delay_length > write_index_f32 {
            self.max_length + write_index_f32 - delay_length
        } else {
            write_index_f32 - delay_length
        };
        read_index
    }
}
