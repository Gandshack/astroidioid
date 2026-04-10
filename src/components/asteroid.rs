use glam::Vec3;
use phantom_core::ecs::Component;

pub struct Asteroid {
    pub velocity: Vec3,
    pub speed: f32,
    pub stage: u32,
    pub radius: u32,
}

impl Component for Asteroid {}

impl Asteroid {
    pub fn new(stage: u32) -> Self {
        Self {
            velocity: Vec3::ZERO,
            speed: 20.0,
            stage,
            radius: 0,
        }
    }
}
