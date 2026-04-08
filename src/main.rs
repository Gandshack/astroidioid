mod components;
mod config;
mod game;
mod input;
mod script;
mod scripts;

use phantom_core::ecs::{World, components::Transform};
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::RaylibTexture2D;
use raylib::{color::Color, prelude::RaylibDraw};

use crate::components::sprite::Sprite;
use crate::config::Config;
use crate::input::Input;
use crate::script::Script;
use crate::scripts::asteroid_manager::AsteroidManager;
use crate::scripts::bullet_manager::BulletManager;
use crate::scripts::collision_manager::CollisionManager;
use crate::scripts::player::Player;
use crate::scripts::screen_wrapper::ScreenWrapper;

fn main() {
    let mut input = Input::new();
    let mut scripts: Vec<Box<dyn Script>> = vec![
        Box::new(ScreenWrapper::new()),
        Box::new(Player::new()),
        Box::new(AsteroidManager::new()),
        Box::new(BulletManager::new()),
        Box::new(CollisionManager::new()),
    ];

    let mut config = Config {
        width: 1280,
        height: 720,
        delta_time: 0.0,
    };

    let (mut rl, thread) = raylib::init().size(config.width, config.height).build();
    rl.set_window_monitor(0);

    let mut world = World::new();

    for script in &mut scripts {
        script.start(&mut world, &mut input, &config);
    }

    let entities = &world.query_with2::<Transform, Sprite>();

    //Load Textures
    for entity in entities {
        let sprite_component = world.get_component_mut::<Sprite>(*entity).unwrap();
        sprite_component.texture =
            Some(rl.load_texture(&thread, sprite_component.filename).unwrap());
    }

    while !rl.window_should_close() {
        config.delta_time = rl.get_frame_time();
        //Gather input
        input.w = rl.is_key_down(raylib::ffi::KeyboardKey::KEY_W);
        input.a = rl.is_key_down(raylib::ffi::KeyboardKey::KEY_A);
        input.s = rl.is_key_down(raylib::ffi::KeyboardKey::KEY_S);
        input.d = rl.is_key_down(raylib::ffi::KeyboardKey::KEY_D);
        input.space = rl.is_key_pressed(raylib::ffi::KeyboardKey::KEY_SPACE);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        //Run Scripts Update
        for script in &mut scripts {
            script.update(&mut world, &mut input, &config);
        }
        //Draw Sprites
        for entity in entities {
            let transform = world.get_component::<Transform>(*entity).unwrap();
            let sprite = world.get_component::<Sprite>(*entity).unwrap();
            d.draw_texture_pro(
                &sprite.texture.as_ref().unwrap(),
                Rectangle::new(
                    0.0,
                    0.0,
                    sprite.texture.as_ref().unwrap().width() as f32,
                    sprite.texture.as_ref().unwrap().height as f32,
                ),
                Rectangle::new(
                    transform.position.x,
                    transform.position.y,
                    sprite.texture.as_ref().unwrap().width() as f32,
                    sprite.texture.as_ref().unwrap().height as f32,
                ),
                Vector2::new(
                    sprite.texture.as_ref().unwrap().width() as f32 / 2.0,
                    sprite.texture.as_ref().unwrap().height as f32 / 2.0,
                ),
                transform
                    .rotation
                    .to_euler(glam::EulerRot::XYZ)
                    .2
                    .to_degrees(),
                Color::WHITE,
            );
        }
    }
}
