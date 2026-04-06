use phantom_core::ecs::Component;

use crate::script::Script;

pub struct ScriptComponent {
    script: Box<dyn Script>,
}

impl Component for ScriptComponent {}
