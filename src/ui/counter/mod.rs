/// The Bevy Plugin that orchestrates everything
pub use plugin::UICounterPlugin;

/// Following the NgRx architecture: https://dev.to/jagadeeshmusali/angular-ngrx-state-management-lifecycle-simplified-khl
///
///                 /--------Store
///                /           ^
///               /            |
/// Template<----/          Reducer
///     ^  \                  ^
///     |   \-----\           |
/// Component      \------->Action
///

mod plugin;

mod actions;
mod facade;
mod reducer;
mod store;
mod template;
mod component;

