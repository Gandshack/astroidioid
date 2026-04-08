use phantom_core::ecs::{World, components::Transform};

use crate::{components::sprite::Sprite, config::Config, input::Input, script::Script};

pub struct ScreenWrapper {}

impl ScreenWrapper {
    pub fn new() -> Self {
        Self {}
    }
}

impl Script for ScreenWrapper {
    fn start(&mut self, world: &mut World, input: &mut Input, config: &Config) {}
    fn update(&mut self, world: &mut World, input: &mut Input, config: &Config) {
        for id in world.query_with2::<Transform, Sprite>() {
            let position = world
                .get_component::<Transform>(id)
                .unwrap()
                .position
                .clone();

            if position.x >= config.width as f32 {
                let transform = world.get_component_mut::<Transform>(id).unwrap();
                transform.position.x = 0.0;
            } else if position.x <= 0.0 {
                let transform = world.get_component_mut::<Transform>(id).unwrap();
                transform.position.x = config.width as f32;
            }

            if position.y >= config.height as f32 {
                let transform = world.get_component_mut::<Transform>(id).unwrap();
                transform.position.y = 0.0;
            } else if position.y <= 0.0 {
                let transform = world.get_component_mut::<Transform>(id).unwrap();
                transform.position.y = config.height as f32;
            }
        }
    }
}
