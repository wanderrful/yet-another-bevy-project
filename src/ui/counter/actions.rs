use bevy::ecs::component::Component;

/// Event for mutating the counter's value
pub struct CounterActionIncrement {
    // By how much to increment
    pub delta: i32
}

pub struct CounterActionMenuSetVisibility {
    // Whether the menu should now be hidden
    pub visible: bool
}
