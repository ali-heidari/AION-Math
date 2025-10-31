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

    pub fn sma(&mut self, label: u32, count: usize, new_value: f32) -> Option<f32> {
        let frame: &mut Frame = self.get_frame(label);

        frame.values.push_back(new_value);

        if frame.values.len() < count {
            return None;
        }

        if frame.values.len() == count {
            let values = frame.values.as_slices();
            frame.calculated_value = Math::sma(values.0);
            return Some(frame.calculated_value);
        }

        let first_value = frame.values.pop_front().unwrap();
        frame.count = count;
        frame.calculated_value += (new_value - first_value) / frame.count as f32;

        Some(frame.calculated_value)
    }

    pub fn ema(&mut self, label: u32, count: usize, new_value: f32) -> Option<f32> {
        let frame: &mut Frame = self.get_frame(label);

        frame.values.push_back(new_value);

        if frame.values.len() < count {
            return None;
        }

        if frame.values.len() == count {
            let values = frame.values.as_slices();
            frame.calculated_value = Math::sma(values.0);
            return Some(frame.calculated_value);
        }

        let smoothing_factor: f32 = 2.0 / (count + 1) as f32;
        frame.calculated_value =
            (new_value * smoothing_factor) + (frame.calculated_value * (1.0 - smoothing_factor));

        Some(frame.calculated_value)
    }

    pub fn macd(
        &mut self,
        label: u32,
        fast_ema: usize,
        slow_ema: usize,
        signal_line_ema: usize,
        new_value: f32,
    ) -> Option<(f32, f32)> {
        let fast_ema = self.ema(label, fast_ema, new_value);
        let slow_ema = self.ema(label + 1, slow_ema, new_value);

        slow_ema?;

        let macd_line = fast_ema.unwrap() - slow_ema.unwrap();

        let signal_line_ema = self.ema(label + 2, signal_line_ema, macd_line);

        signal_line_ema?;

        println!("asds {:?}", (fast_ema, slow_ema, signal_line_ema));
        Some((macd_line, signal_line_ema.unwrap()))
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
        assert_eq!(result, None);
        result = continuous_math.sma(1, 5, 11.0);
        assert_eq!(result, None);
        result = continuous_math.sma(1, 5, 12.0);
        assert_eq!(result, None);
        result = continuous_math.sma(1, 5, 11.0);
        assert_eq!(result, None);
        result = continuous_math.sma(1, 5, 14.0);
        assert_eq!(result, Some(11.6));
        result = continuous_math.sma(1, 5, 15.0);
        assert_eq!(result, Some(12.6));
        result = continuous_math.sma(1, 5, 13.0);
        assert_eq!(result, Some(13.0));
    }

    #[test]
    fn ema() {
        let mut continuous_math: ContinuousMath = ContinuousMath::new();
        let mut result = continuous_math.ema(1, 10, 22.0);
        assert_eq!(result, None);
        result = continuous_math.ema(1, 10, 24.0);
        assert_eq!(result, None);
        result = continuous_math.ema(1, 10, 26.0);
        assert_eq!(result, None);
        result = continuous_math.ema(1, 10, 25.0);
        assert_eq!(result, None);
        result = continuous_math.ema(1, 10, 28.0);
        assert_eq!(result, None);
        result = continuous_math.ema(1, 10, 27.0);
        assert_eq!(result, None);
        result = continuous_math.ema(1, 10, 29.0);
        assert_eq!(result, None);
        result = continuous_math.ema(1, 10, 30.0);
        assert_eq!(result, None);
        result = continuous_math.ema(1, 10, 29.0);
        assert_eq!(result, None);
        result = continuous_math.ema(1, 10, 32.0);
        assert_eq!(result, Some(27.2));
        result = continuous_math.ema(1, 10, 35.0);
        assert_eq!(result, Some(28.618181));
        result = continuous_math.ema(1, 10, 33.0);
        assert_eq!(result, Some(29.414875));
    }

    #[test]
    fn macd() {
        let mut continuous_math: ContinuousMath = ContinuousMath::new();
        let mut result = continuous_math.macd(1, 12, 26, 9, 55.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 57.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 58.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 60.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 62.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 61.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 63.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 64.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 65.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 67.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 66.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 68.0);
        result = continuous_math.macd(1, 12, 26, 9, 69.0);
        result = continuous_math.macd(1, 12, 26, 9, 70.0);
        result = continuous_math.macd(1, 12, 26, 9, 72.0);
        result = continuous_math.macd(1, 12, 26, 9, 71.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 73.0);
        result = continuous_math.macd(1, 12, 26, 9, 72.0);
        result = continuous_math.macd(1, 12, 26, 9, 74.0);
        result = continuous_math.macd(1, 12, 26, 9, 75.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 76.0);
        result = continuous_math.macd(1, 12, 26, 9, 78.0);
        result = continuous_math.macd(1, 12, 26, 9, 79.0);
        result = continuous_math.macd(1, 12, 26, 9, 80.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 82.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 81.0);
        result = continuous_math.macd(1, 12, 26, 9, 80.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 78.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 77.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 76.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 75.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 77.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 76.0);
        assert_eq!(result, None);
        result = continuous_math.macd(1, 12, 26, 9, 79.0);
        assert_eq!(result, Some((4.05381, 5.473577)));
        result = continuous_math.macd(1, 12, 26, 9, 80.0);
        assert_eq!(result, Some((4.000931, 5.179048)));
    }
}
