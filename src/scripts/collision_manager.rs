use crate::components::asteroid::Asteroid;
use crate::components::bullet::Bullet;
use crate::components::sprite::Sprite;
use crate::script::Script;
use crate::{config::Config, input::Input};
use phantom_core::ecs::World;
use phantom_core::ecs::components::Transform;
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
                let sprite_size = world
                    .get_component::<Sprite>(asteroid)
                    .unwrap()
                    .texture
                    .as_ref()
                    .unwrap()
                    .height;
                let collision_radius = sprite_size / 2;
                if distance_to_asteroid <= collision_radius as f32 {
                    println!("BOOM!")
                }
            }
        }
    }
}
