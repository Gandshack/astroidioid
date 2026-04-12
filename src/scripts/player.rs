use crate::{
    audio::Audio,
    components::{player_component::PlayerComponent, sprite::Sprite},
    config::Config,
    input::Input,
};
use glam::{Quat, Vec3};
use phantom_core::ecs::{Component, World, components::Transform};

use crate::script::Script;

pub struct Player {
    fly_speed: f32,
    rotation_speed: f32,
    velocity: Vec3,
    max_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            fly_speed: 1000.0,
            rotation_speed: 0.001,
            velocity: Vec3::ZERO,
            max_speed: 1000.0,
        }
    }
}

impl Script for Player {
    fn start(&mut self, world: &mut World, input: &mut Input, config: &Config, audio: &Audio) {}
    fn update(&mut self, world: &mut World, input: &mut Input, config: &Config, audio: &Audio) {
        if let Some(player_id) = world.query_with::<PlayerComponent>().first() {
            let transform = world.get_component_mut::<Transform>(*player_id).unwrap();

            let rotation_input =
                (input.d as i32 as f32 - input.a as i32 as f32) * self.rotation_speed;

            let rotation = transform.rotation.to_euler(glam::EulerRot::XYZ).2;
            transform.rotation = Quat::from_rotation_z(rotation + rotation_input);

            let forward = transform.rotation * Vec3::NEG_Y;
            if input.w {
                self.velocity += forward * self.fly_speed * config.delta_time;
            }

            let dampening: f32 = 0.01;
            self.velocity *= dampening.powf(config.delta_time);
            self.velocity = self.velocity.clamp_length_max(self.max_speed);
            transform.position += self.velocity * config.delta_time;
        }
    }
}
