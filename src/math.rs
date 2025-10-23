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

    pub fn variance(numbers: Vec<f32>) -> f32 {
        let mean: f32 = numbers.iter().sum::<f32>() / numbers.len() as f32;
        let total_difference_from_mean: f32 = numbers.iter().map(|x| (x - mean).powi(2)).sum();

        let variance = total_difference_from_mean / numbers.len() as f32;

        variance
    }

    pub fn variance_of_ratios(numbers: Vec<f32>) -> f32 {
        let total: f32 = numbers.iter().sum();
        let ratios: Vec<f32> = numbers.iter().map(|x| x / total).collect();

        let variance = Math::variance(ratios);

        variance
    }
}
