use std::f32::consts::PI;

use crate::components::asteroid::Asteroid;
use crate::components::bullet::Bullet;
use crate::components::sprite::Sprite;
use crate::script::Script;
use crate::{config::Config, input::Input};
use glam::{Quat, Vec3};
use phantom_core::ecs::components::Transform;
use phantom_core::ecs::{World, world};
use rand::{RngExt, rng};
pub struct CollisionManager {}

impl CollisionManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl Script for CollisionManager {
    fn start(&mut self, world: &mut World, input: &mut Input, config: &Config) {}
    fn update(&mut self, world: &mut World, input: &mut Input, config: &Config) {
        // Bullet to asteroid
        for bullet in world.query_with::<Bullet>() {
            let bullet_position = world.get_component::<Transform>(bullet).unwrap().position;
            for asteroid in world.query_with::<Asteroid>() {
                let asteroid_position =
                    world.get_component::<Transform>(asteroid).unwrap().position;
                let distance_to_asteroid = bullet_position.distance(asteroid_position);
                let collision_radius = world.get_component::<Asteroid>(asteroid).unwrap().radius;
                let stage = world
                    .get_component::<Asteroid>(asteroid)
                    .unwrap()
                    .stage
                    .clone();
                if distance_to_asteroid <= collision_radius as f32 {
                    match stage {
                        0 => {
                            //spawn 2 stage 1 asteroids (Medium Sized)
                            let texture_path = "src/sprites/asteroid_medium.png";
                            let size = 32;
                            let position = asteroid_position;
                            let new_stage = 1;
                            spawn_asteroid(
                                world.spawn(),
                                texture_path,
                                size,
                                position,
                                new_stage,
                                world,
                            );
                            spawn_asteroid(
                                world.spawn(),
                                texture_path,
                                size,
                                position,
                                new_stage,
                                world,
                            );
                        }
                        1 => {
                            //spawn 2 stage 1 asteroids (Small Sized)
                            //                             //spawn 2 stage 1 asteroids (Medium Sized)
                            let texture_path = "src/sprites/asteroid_small.png";
                            let size = 16;
                            let position = asteroid_position;
                            let new_stage = 2;
                            spawn_asteroid(
                                world.spawn(),
                                texture_path,
                                size,
                                position,
                                new_stage,
                                world,
                            );
                            spawn_asteroid(
                                world.spawn(),
                                texture_path,
                                size,
                                position,
                                new_stage,
                                world,
                            );
                        }
                        _ => {}
                    }
                    world.destroy(asteroid);
                    world.destroy(bullet);
                    break;
                }
            }
        }
    }
}

fn random_rotation() -> f32 {
    rng().random_range(0.0..=(2.0 * PI)) as f32
}

fn spawn_asteroid(
    id: u32,
    texture_path: &'static str,
    size: u32,
    position: Vec3,
    new_stage: u32,
    world: &mut World,
) {
    world.add_component::<Sprite>(id, Sprite::new(texture_path));
    world.add_component::<Asteroid>(id, Asteroid::new(new_stage));
    let asteroid_component = world.get_component_mut::<Asteroid>(id).unwrap();
    asteroid_component.speed = asteroid_component.speed * 2.0;
    asteroid_component.radius = size / 2; //32 pix hardcoded to avoid crash when searching sprite size directly
    let transform_one = world.get_component_mut::<Transform>(id).unwrap();
    transform_one.position = position;
    transform_one.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, random_rotation());
}
