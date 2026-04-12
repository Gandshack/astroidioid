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

pub struct AsteroidManager {}

impl AsteroidManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl Script for AsteroidManager {
    fn start(&mut self, world: &mut World, input: &mut Input, config: &Config) {}

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
