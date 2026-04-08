use crate::{Input, config::Config};
use phantom_core::ecs::{Component, World};

pub trait Script {
    fn start(&mut self, world: &mut World, input: &mut Input, config: &Config) {}
    fn update(&mut self, world: &mut World, input: &mut Input, config: &Config) {}
}
