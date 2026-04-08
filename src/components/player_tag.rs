use glam::Vec3;
use phantom_core::ecs::Component;

pub struct PlayerTag {}

impl Component for PlayerTag {}

impl PlayerTag {
    pub fn new() -> Self {
        Self {}
    }
}
