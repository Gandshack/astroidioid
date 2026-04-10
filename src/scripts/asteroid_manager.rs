use std::f32::consts::PI;

use crate::{
    components::{asteroid::Asteroid, sprite::Sprite},
    config::Config,
    input::Input,
    script::Script,
};
use glam::{Quat, Vec3};
use phantom_core::{ecs::World, ecs::components::Transform};
use rand::{Rng, RngExt, rng};

pub struct AsteroidManager {
    asteroid_count: i32,
}

impl AsteroidManager {
    pub fn new() -> Self {
        Self { asteroid_count: 10 }
    }
}

impl Script for AsteroidManager {
    fn start(&mut self, world: &mut World, input: &mut Input, config: &Config) {
        for _asteroid in 0..self.asteroid_count {
            let id = world.spawn();

            world.add_component::<Sprite>(id, Sprite::new("src/sprites/asteroid_big.png"));
            world.add_component::<Asteroid>(id, Asteroid::new(0));
            let asteroid = world.get_component_mut::<Asteroid>(id).unwrap();
            asteroid.radius = 64 / 2; //64 pix hardcoded to avoid crash when searching sprite size directly

            let transform = world.get_component_mut::<Transform>(id).unwrap();
            let x = rng().random_range(0..config.width);
            let y = rng().random_range(0..config.height);
            transform.position = Vec3 {
                x: x as f32,
                y: y as f32,
                z: 0.0,
            };

            transform.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, random_rotation());
        }
    }

    fn update(&mut self, world: &mut World, input: &mut Input, config: &Config) {
        let asteroid_ids = world.query_with::<Asteroid>();
        for id in asteroid_ids.iter() {
            let mut forward = Vec3::ZERO;
            {
                let transform = world.get_component_mut::<Transform>(*id).unwrap();
                forward = transform.rotation * Vec3::NEG_Y;
            }
            let mut velocity = Vec3::ZERO;
            {
                let asteroid = world.get_component_mut::<Asteroid>(*id).unwrap();
                velocity = forward * asteroid.speed * config.delta_time;
                asteroid.velocity = velocity;
            }
            let transform = world.get_component_mut::<Transform>(*id).unwrap();
            transform.position += velocity;
        }
    }
}

fn random_rotation() -> f32 {
    rng().random_range(0.0..=(2.0 * PI)) as f32
}
