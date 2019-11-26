use crate::init_field;
use crate::models::field::Field;
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

pub enum RayType {
    Slide { max_len: f32 },
    Rotate { start_angle: f32, end_angle: f32 },
}

pub struct RayLocal<'a> {
    pub ray_type: RayType,
    pub x: f32,
    pub y: f32,
    pub start_at: Instant,
    pub duration: u64, // NOTE: milliseconds
    pub callback: &'a dyn Fn(Self),
}

impl<'a> RayLocal<'a> {
    fn slide(x: f32, y: f32, duration: u64, max_len: f32, callback: &'a dyn Fn(Self)) -> Self {
        Self {
            ray_type: RayType::Slide { max_len },
            x,
            y,
            start_at: Instant::now(),
            duration,
            callback,
        }
    }
    fn rotate(x: f32, y: f32, duration: u64, start_angle: f32, end_angle: f32, &'a dyn Fn(Self)) -> Self {
        Self {
            ray_type: RayType::Rotate {
                start_angle,
                end_angle,
            },
            x,
            y,
            start_at: Instant::now(),
            duration,
            callback,
        }
    }
}
