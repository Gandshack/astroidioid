use crate::{Input, audio::Audio, config::Config};
use phantom_core::ecs::{Component, World};

pub trait Script {
    fn start(&mut self, world: &mut World, input: &mut Input, config: &Config, audio: &Audio) {}
    fn update(&mut self, world: &mut World, input: &mut Input, config: &Config, audio: &Audio) {}
}
