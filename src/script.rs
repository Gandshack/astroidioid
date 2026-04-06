use crate::Input;
use phantom_core::ecs::{Component, World};

pub trait Script: Component {
    fn start(&mut self, world: &mut World, input: &mut Input) {}
    fn update(&mut self, world: &mut World, input: &mut Input) {}
}
