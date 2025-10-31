use std::collections::HashMap;

use crate::{
    continuous_math::{ContinuousMath, frame::Frame},
    math::Math,
};

impl ContinuousMath {
    pub fn new() -> Self {
        Self {
            frames: HashMap::new(),
        }
    }

    fn get_frame(&mut self, label: u32) -> &mut Frame {
        let frame = Frame::default();
        self.frames.entry(label).or_insert(frame)
    }

    pub fn average(&mut self, label: u32, new_value: f32) -> f32 {
        let frame: &mut Frame = self.get_frame(label);
        let avg = frame.calculated_value
            + (new_value - frame.calculated_value) / (frame.count + 1) as f32;
        frame.calculated_value = avg;
        frame.count += 1;
        avg
    }

    pub fn calc_avg_and_trend(&mut self, label: u32, new_value: f32, range: f32) -> (f32, i32) {
        let frame: &mut Frame = self.get_frame(label);
        let prev_avg = frame.calculated_value;
        let avg = self.average(label, new_value);
        let avg_dif = (avg - prev_avg).abs();
        let trend = if avg_dif > 0.0 + range {
            1
        } else if avg_dif < 0.0 - range {
            -1
        } else {
            0
        };
        (avg, trend)
    }

    pub fn sma(&mut self, label: u32, count: usize, new_value: f32) -> f32 {
        let frame: &mut Frame = self.get_frame(label);

        if frame.values.len() < count - 1 {
            frame.values.push_back(new_value);
            return -1.0;
        }

        if frame.values.len() == count - 1 {
            frame.values.push_back(new_value);
            let values = frame.values.as_slices();
            frame.calculated_value = Math::sma(values.0);
            return frame.calculated_value;
        }

        let first_value = frame.values.pop_front().unwrap();
        frame.count = count;
        frame.calculated_value += (new_value - first_value) / frame.count as f32;
        frame.values.push_back(new_value);

        frame.calculated_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_correct() {
        let mut continuous_math: ContinuousMath = ContinuousMath::new();
        continuous_math.average(1, 1.0);
        continuous_math.average(1, 2.0);
        continuous_math.average(1, 3.0);
        continuous_math.average(1, 4.0);
        continuous_math.average(1, 5.0);
        continuous_math.average(1, 6.0);
        let result = continuous_math.average(1, 7.0);
        assert_eq!(result, 4.0);
    }

    #[test]
    fn average_incorrect() {
        let mut continuous_math: ContinuousMath = ContinuousMath::new();
        continuous_math.average(1, 1.0);
        continuous_math.average(1, 2.0);
        continuous_math.average(1, 3.0);
        continuous_math.average(1, 4.0);
        continuous_math.average(1, 5.0);
        continuous_math.average(1, 6.0);
        let result = continuous_math.average(1, 7.0);
        assert_ne!(result, 5.0);
    }

    #[test]
    fn sma() {
        let mut continuous_math: ContinuousMath = ContinuousMath::new();
        let mut result = continuous_math.sma(1, 5, 10.0);
        assert_eq!(result, -1.0);
        result = continuous_math.sma(1, 5, 11.0);
        assert_eq!(result, -1.0);
        result = continuous_math.sma(1, 5, 12.0);
        assert_eq!(result, -1.0);
        result = continuous_math.sma(1, 5, 11.0);
        assert_eq!(result, -1.0);
        result = continuous_math.sma(1, 5, 14.0);
        assert_eq!(result, 11.6);
        result = continuous_math.sma(1, 5, 15.0);
        assert_eq!(result, 12.6);
        result = continuous_math.sma(1, 5, 13.0);
        assert_eq!(result, 13.0);
    }
}
