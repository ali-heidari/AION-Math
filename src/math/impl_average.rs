use crate::math::Math;

impl Math {
    pub fn average(values: &[f32]) -> f32 {
        values.iter().sum::<f32>() / values.len() as f32
    }

    pub fn sma(values: &[f32]) -> f32 {
        Math::average(values)
    }
}
