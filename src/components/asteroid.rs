use glam::Vec3;
use phantom_core::ecs::Component;

pub struct Asteroid {
    pub velocity: Vec3,
    pub speed: f32,
}

impl Component for Asteroid {}

impl Asteroid {
    pub fn new() -> Self {
        Self {
            velocity: Vec3::ZERO,
            speed: 10.0,
        }
    }
}
