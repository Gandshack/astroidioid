use phantom_core::ecs::{Component, World};

pub trait Script: Component {
    fn start(&mut self, world: &mut World) {}
    fn update(&mut self, world: &mut World) {}
}
