use phantom_core::ecs::Component;
use raylib::{RaylibHandle, RaylibThread, color::Color, texture::Texture2D};

pub struct Sprite {
    pub filename: &'static str,
    pub texture: Option<Texture2D>,
    pub tint: Color,
}

impl Component for Sprite {}

impl Sprite {
    pub fn new(filename: &'static str) -> Self {
        Self {
            filename,
            texture: None,
            tint: Color::WHITE,
        }
    }
}
