use crate::audio::Audio;
use crate::components::game_state::GameState;
use crate::components::player_component::PlayerComponent;
use crate::components::sprite::Sprite;
use crate::script::Script;
use crate::{config::Config, input::Input};
use glam::Vec3;
use phantom_core::ecs::World;
use phantom_core::ecs::components::Transform;

pub struct GameManager {
    game_over: bool,
}

impl GameManager {
    pub fn new() -> Self {
        Self { game_over: false }
    }
}

impl Script for GameManager {
    fn start(&mut self, world: &mut World, input: &mut Input, config: &Config, audio: &Audio) {}
    fn update(&mut self, world: &mut World, input: &mut Input, config: &Config, audio: &Audio) {
        let game_state_id = world.query_with::<GameState>().first().unwrap().clone();
        let game_state = world.get_component::<GameState>(game_state_id).unwrap();

        if world.query_with::<PlayerComponent>().first().is_none() && game_state.lives != 0 {
            let id = world.spawn();
            world.add_component::<PlayerComponent>(id, PlayerComponent::new());
            world.add_component::<Sprite>(id, Sprite::new("assets/sprites/player.png"));
            //Center Player
            let transform = world.get_component_mut::<Transform>(id).unwrap();
            transform.position = Vec3 {
                x: config.width as f32 / 2.0,
                y: config.height as f32 / 2.0,
                z: 0.0,
            }
        } else if game_state.lives == 0 && self.game_over == false {
            self.game_over = true;
            let id = world.spawn();
            world.add_component::<Sprite>(id, Sprite::new("assets/sprites/game_over.png"));
            //Center Player
            let transform = world.get_component_mut::<Transform>(id).unwrap();
            transform.position = Vec3 {
                x: config.width as f32 / 2.0,
                y: config.height as f32 / 2.0,
                z: 0.0,
            }
        }
    }
}
