use crate::math::Math;

impl Math {
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
