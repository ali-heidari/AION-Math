use crate::math::Math;

impl Math {
    pub fn average(&mut self, values: &[f32]) -> f32 {
        values.iter().sum::<f32>() / values.len() as f32
    }

    pub fn sma(&self, values: &[f32]) {
        self.average(values)
    }
}
