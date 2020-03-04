use amethyst::{
    core::math::Vector3,
};

use crate::components::SegmentDirection;

pub struct SnakeTracker {
    pub last_head_pos: Vector3<f32>,
    pub last_head_dir: SegmentDirection,
    pub food_available: bool,
    pub score: u64,
}