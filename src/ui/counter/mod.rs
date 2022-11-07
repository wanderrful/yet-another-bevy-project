/// Send this event to toggle the visibility of this widget
pub use actions::CounterActionMenuSetVisibility;
/// The Bevy Plugin that orchestrates everything
pub use plugin::UICounterPlugin;

mod plugin;

mod actions;
mod bindings;
mod facade;
mod reducer;
mod store;
mod view;