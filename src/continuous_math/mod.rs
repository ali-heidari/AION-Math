mod frame;
mod impl_average;

use std::collections::HashMap;

use crate::continuous_math::frame::Frame;

#[derive(Default)]
pub struct ContinuousMath {
    frames: HashMap<u32, Frame>,
}
