use crate::continuous_average::continuous_average;

pub struct Math {
    previous_avg: f32,
    previous_count: u32,
}

impl Math {
    pub fn new() -> Self {
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

    pub fn calc_avg_and_trend(&mut self, new_value: f32, range: f32) -> (f32, i32) {
        let prev_avg = self.previous_avg;
        let avg = self.average(new_value);
        let avg_dif = avg - prev_avg;
        let trend = if avg_dif > 0.0 + range {
            1
        } else if avg_dif < 0.0 - range {
            -1
        } else {
            0
        };
        (avg, trend)
    }
}
