use std::collections::VecDeque;


#[derive(Default)]
pub struct Frame {
    pub count: usize,
    pub calculated_value: f32,
    pub values: VecDeque<f32>
}