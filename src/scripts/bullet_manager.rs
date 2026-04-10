use crate::components::bullet::Bullet;
use crate::components::player_component::PlayerComponent;
use crate::components::sprite::Sprite;
use crate::script::Script;
use crate::{config::Config, input::Input};
use glam::Vec3;
use phantom_core::ecs::World;
use phantom_core::ecs::components::Transform;
use raylib::prelude::RaylibTexture2D;

pub struct BulletManager {
    max_bullets: u32,
    next_bullet: usize,
    offscreen_position: Vec3,
}

impl BulletManager {
    pub fn new() -> Self {
        Self {
            max_bullets: 10,
            next_bullet: 0,
            offscreen_position: Vec3::new(-9.0, -9.0, -9.0),
        }
    }
}

impl Script for BulletManager {
    fn start(&mut self, world: &mut World, input: &mut Input, config: &Config) {}
    fn update(&mut self, world: &mut World, input: &mut Input, config: &Config) {
        let bullet_ids = world.query_with::<Bullet>();
        // Spawn bullets on player
        if input.space {
            if let Some(player_id) = world.query_with::<PlayerComponent>().first() {
                let bullet_id = world.spawn();

                world.add_component::<Sprite>(bullet_id, Sprite::new("src/sprites/bullet.png"));
                world.add_component::<Bullet>(bullet_id, Bullet::new());
                let mut player_forward = Vec3::ZERO;
                {
                    let player_transform = world.get_component::<Transform>(*player_id).unwrap();
                    player_forward = player_transform.rotation * Vec3::NEG_Y;
                }

                {
                    let bullet_component = world.get_component_mut::<Bullet>(bullet_id).unwrap();
                    bullet_component.forward = player_forward.clone();
                    bullet_component.lifetime = 0.0;
                }

                let player_transform = world.get_component::<Transform>(*player_id).unwrap();
                let player_position = player_transform.position.clone();

                let player_sprite = world.get_component::<Sprite>(*player_id).unwrap();
                let sprite_size = player_sprite.texture.as_ref().unwrap().height().clone();
                let margin = 10.0;
                let offset = ((sprite_size as f32 / 2.0) + margin) * player_forward;

                let bullet_transform = world.get_component_mut::<Transform>(bullet_id).unwrap();
                bullet_transform.position = player_position + offset;
            }
        }

        // Move Bullets
        for bullet in bullet_ids {
            let bullet_component = world.get_component_mut::<Bullet>(bullet).unwrap();
            let velocity = bullet_component.forward * bullet_component.speed;
            bullet_component.velocity = velocity.clone();

            let bullet_transform = world.get_component_mut::<Transform>(bullet).unwrap();
            bullet_transform.position += velocity * config.delta_time;

            let bullet_component = world.get_component_mut::<Bullet>(bullet).unwrap();
            bullet_component.lifetime += config.delta_time;
            if bullet_component.lifetime >= bullet_component.max_lifetime {
                world.destroy(bullet);
                // bullet_component.lifetime = 0.0;
                // bullet_component.velocity = Vec3::ZERO;
                // bullet_component.forward = Vec3::ZERO;
                // let bullet_transform = world.get_component_mut::<Transform>(bullet).unwrap();
                // bullet_transform.position = self.offscreen_position;
            }
        }
    }
}
