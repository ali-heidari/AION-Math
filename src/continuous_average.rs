pub fn continuous_average(previous_avg: f32, previous_count: u32, new_value: f32) -> f32 {
    previous_avg + (new_value - previous_avg) / (previous_count + 1) as f32
}
