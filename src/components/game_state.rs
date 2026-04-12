use glam::bool;
use phantom_core::ecs::Component;

pub struct GameState {
    pub level: i32,
    pub lives: u32,
    pub score: i32,
}

impl Component for GameState {}

impl GameState {
    pub fn new() -> Self {
        Self {
            level: 49,
            lives: 3,
            score: 0,
        }
    }
}
