use glam::Vec3;
use phantom_core::ecs::Component;

pub struct Bullet {
    pub velocity: Vec3,
    pub speed: f32,
    pub max_lifetime: f32,
    pub lifetime: f32,
    pub forward: Vec3,
}

impl Component for Bullet {}

impl Bullet {
    pub fn new() -> Self {
        Self {
            velocity: Vec3::ZERO,
            speed: 300.0,
            max_lifetime: 2.0,
            lifetime: 0.0, //secs
            forward: Vec3::ZERO,
        }
    }
}
