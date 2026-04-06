use crate::{input::Input, sprite::Sprite};
use glam::{Quat, Vec3};
use phantom_core::{
    constants::constants::INVALID,
    ecs::{Component, World, components::Transform},
};

use crate::script::Script;

pub struct Player {
    id: u32,
    fly_speed: f32,
    roation_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            id: INVALID,
            fly_speed: 0.01,
            roation_speed: 0.001,
        }
    }
}

impl Component for Player {}
impl Script for Player {
    fn start(&mut self, world: &mut World, input: &mut Input) {
        self.id = world.spawn();

        world.add_component::<Sprite>(
            self.id,
            Sprite::new("/home/osii/Dev/Rust/astroidioid/src/sprites/player.png"),
        );
    }
    fn update(&mut self, world: &mut World, input: &mut Input) {
        let transform = world.get_component_mut::<Transform>(self.id).unwrap();

        let move_input = (input.s as i32 as f32 - input.w as i32 as f32) * self.fly_speed;
        let rotation_input = (input.d as i32 as f32 - input.a as i32 as f32) * self.roation_speed;

        let rotation = transform.rotation.to_euler(glam::EulerRot::XYZ).2;
        transform.rotation = Quat::from_rotation_z(rotation + rotation_input);

        let forward = transform.rotation * Vec3::Y;
        transform.position += forward * move_input;
    }
}
