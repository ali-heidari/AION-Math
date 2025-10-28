use std::collections::HashMap;

mod impl_average;

pub struct Frame {
    count: u32,
    value: f32,
}

pub struct ContinuousMath {
    frames: HashMap<u32, Frame>,
}