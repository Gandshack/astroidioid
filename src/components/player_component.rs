use phantom_core::ecs::Component;

pub struct PlayerComponent {
    pub radius: u32,
}

impl Component for PlayerComponent {}

impl PlayerComponent {
    pub fn new() -> Self {
        Self {
            radius: 32 / 2, //32px hardcoded to avoid crash when checking sprite directly
        }
    }
}
