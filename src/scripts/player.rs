use crate::sprite::Sprite;
use phantom_core::{
    constants::constants::INVALID,
    ecs::{Component, World, components::Transform},
};

use crate::script::Script;

pub struct Player {
    id: u32,
}
impl Player {
    pub fn new() -> Self {
        Self { id: INVALID }
    }
}
impl Component for Player {}
impl Script for Player {
    fn start(&mut self, world: &mut World) {
        self.id = world.spawn();

        world.add_component::<Sprite>(
            self.id,
            Sprite::new("/home/osii/Dev/Rust/astroidioid/src/sprites/player.png"),
        );
    }
    fn update(&mut self, world: &mut World) {
        let transform = world.get_component_mut::<Transform>(self.id).unwrap();
        transform.position.x += 0.01;
    }
}
