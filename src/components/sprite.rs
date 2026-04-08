use phantom_core::ecs::Component;
use raylib::{RaylibHandle, RaylibThread, texture::Texture2D};

pub struct Sprite {
    pub filename: &'static str,
    pub texture: Option<Texture2D>,
}

impl Component for Sprite {}

impl Sprite {
    pub fn new(filename: &'static str) -> Self {
        Self {
            filename,
            texture: None,
        }
    }
}
