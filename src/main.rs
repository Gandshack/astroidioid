mod game;
mod script;
mod script_component;
mod scripts;
mod sprite;

use phantom_core::ecs::{World, components::Transform};
use raylib::{color::Color, prelude::RaylibDraw};

use crate::script::Script;
use crate::scripts::player::Player;
use crate::sprite::Sprite;

fn main() {
    let mut scripts: Vec<Box<dyn Script>> = vec![Box::new(Player::new())];

    let (mut rl, thread) = raylib::init().size(1280, 720).build();

    let mut world = World::new();

    for script in &mut scripts {
        script.start(&mut world);
    }

    let entities = &world.query_with2::<Transform, Sprite>();

    //Load Textures
    for entity in entities {
        let sprite_component = world.get_component_mut::<Sprite>(*entity).unwrap();
        sprite_component.texture =
            Some(rl.load_texture(&thread, sprite_component.filename).unwrap());
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        //Run Scripts Update
        for script in &mut scripts {
            script.update(&mut world);
        }
        //Draw Sprites
        for entity in entities {
            let transform = world.get_component::<Transform>(*entity).unwrap();
            let sprite = world.get_component::<Sprite>(*entity).unwrap();
            d.draw_texture(
                &sprite.texture.as_ref().unwrap(),
                transform.position.x as i32,
                transform.position.y as i32,
                Color::WHITE,
            );
        }
    }
}
