use std::f64::consts::PI;

use crate::audio::Audio;
use crate::components::asteroid::Asteroid;
use crate::components::game_state::GameState;
use crate::components::sprite::Sprite;
use crate::script::Script;
use crate::{config::Config, input::Input};
use glam::{Quat, Vec3};
use phantom_core::ecs::World;
use phantom_core::ecs::components::Transform;
use rand::{RngExt, rng};

pub struct LevelManager {}

impl LevelManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl Script for LevelManager {
    fn start(&mut self, world: &mut World, input: &mut Input, config: &Config, audio: &Audio) {}
    fn update(&mut self, world: &mut World, input: &mut Input, config: &Config, audio: &Audio) {
        //Check for all asteroids dead the bump level
        if world.query_with::<Asteroid>().is_empty() {
            let game_state_id = world.query_with::<GameState>().first().unwrap().clone();
            let game_state = world.get_component_mut::<GameState>(game_state_id).unwrap();
            game_state.level += 1;

            spawn_asteroids(game_state.level, world, config);
        };
    }
}

fn spawn_asteroids(level: i32, world: &mut World, config: &Config) {
    let number_to_spawn = 10 + ((level - 1) * 2);
    for _n in 0..number_to_spawn {
        let id = world.spawn();

        world.add_component::<Sprite>(id, Sprite::new("assets/sprites/asteroid_big.png"));
        let asteroid_component = world.add_component::<Asteroid>(id, Asteroid::new(0));
        asteroid_component.radius = 64 / 2; //64 pix hardcoded to avoid crash when searching sprite size directly
        asteroid_component.speed += 5.0;

        let transform = world.get_component_mut::<Transform>(id).unwrap();
        let exclusion_radius = 64.0;
        transform.position = random_position(config, exclusion_radius);
        transform.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, random_rotation());
    }
}

fn random_rotation() -> f32 {
    rng().random_range(0.0..=(2.0 * PI)) as f32
}

fn random_position(config: &Config, exclusion_radius: f32) -> Vec3 {
    let center = Vec3::new(config.width as f32 / 2.0, config.height as f32 / 2.0, 0.0);
    loop {
        let x = rng().random_range(0..config.width);
        let y = rng().random_range(0..config.height);
        let pos = Vec3 {
            x: x as f32,
            y: y as f32,
            z: 0.0,
        };

        if pos.distance(center) > exclusion_radius {
            return pos;
        }
    }
}
