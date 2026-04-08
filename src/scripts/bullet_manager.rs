use std::thread::spawn;

use crate::components::bullet::Bullet;
use crate::components::player_tag::PlayerTag;
use crate::components::sprite::Sprite;
use crate::script::Script;
use crate::{config::Config, input::Input};
use glam::Vec3;
use phantom_core::ecs::World;
use phantom_core::ecs::components::Transform;

pub struct BulletManager {
    max_bullets: u32,
    bullet_ids: Vec<u32>,
    next_bullet: usize,
    offscreen_position: Vec3,
}

impl BulletManager {
    pub fn new() -> Self {
        Self {
            max_bullets: 10,
            bullet_ids: Vec::new(),
            next_bullet: 0,
            offscreen_position: Vec3::new(-9.0, -9.0, -9.0),
        }
    }
}

impl Script for BulletManager {
    fn start(&mut self, world: &mut World, input: &mut Input, config: &Config) {
        // Spawn all bullets and put them of screen
        for _bullet in 0..self.max_bullets {
            let id = world.spawn();

            world.add_component::<Sprite>(id, Sprite::new("src/sprites/bullet.png"));
            world.add_component::<Bullet>(id, Bullet::new());

            let transform = world.get_component_mut::<Transform>(id).unwrap();
            transform.position = self.offscreen_position;

            self.bullet_ids.push(id);
        }
    }
    fn update(&mut self, world: &mut World, input: &mut Input, config: &Config) {
        // Spawn bullets on player
        if input.space {
            let bullet_id = self.bullet_ids[self.next_bullet];
            let player_id = world.query_with::<PlayerTag>().first().unwrap().clone();

            let mut player_forward = Vec3::ZERO;
            {
                let player_transform = world.get_component::<Transform>(player_id).unwrap();
                player_forward = player_transform.rotation * Vec3::NEG_Y;
            }

            {
                let bullet_component = world.get_component_mut::<Bullet>(bullet_id).unwrap();
                bullet_component.forward = player_forward.clone();
                bullet_component.lifetime = 0.0;
            }

            let mut player_position = Vec3::ZERO;
            {
                let player_transform = world.get_component::<Transform>(player_id).unwrap();
                player_position = player_transform.position.clone();
            }

            let bullet_transform = world.get_component_mut::<Transform>(bullet_id).unwrap();
            bullet_transform.position = player_position;

            self.next_bullet = (self.next_bullet + 1) % self.bullet_ids.len();
        }

        // Move Bullets
        for bullet in &self.bullet_ids {
            let bullet_component = world.get_component_mut::<Bullet>(*bullet).unwrap();
            let velocity = bullet_component.forward * bullet_component.speed;
            bullet_component.velocity = velocity.clone();

            let bullet_transform = world.get_component_mut::<Transform>(*bullet).unwrap();
            bullet_transform.position += velocity * config.delta_time;

            let bullet_component = world.get_component_mut::<Bullet>(*bullet).unwrap();
            bullet_component.lifetime += config.delta_time;
            if bullet_component.lifetime >= bullet_component.max_lifetime {
                bullet_component.lifetime = 0.0;
                bullet_component.velocity = Vec3::ZERO;
                let bullet_transform = world.get_component_mut::<Transform>(*bullet).unwrap();
                bullet_transform.position = self.offscreen_position;
            }
        }
    }
}
