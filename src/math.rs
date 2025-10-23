use crate::continuous_average::continuous_average;

struct Math {
    previous_avg: f32,
    previous_count: u32,
}

impl Math {
    pub fn new() -> Math {
        Self {
            previous_avg: 0.0,
            previous_count: 0,
        }
    }
    pub fn average(&mut self, new_value: f32) -> f32 {
        let avg = continuous_average(self.previous_avg, self.previous_count, new_value);
        self.previous_avg = avg;
        self.previous_count += 1;
        avg
    }
}
