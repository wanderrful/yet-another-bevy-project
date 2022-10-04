/// Send this event to toggle the visibility of this widget
pub use actions::CounterActionMenuSetVisibility;
/// The Bevy Plugin that orchestrates everything
pub use plugin::UICounterPlugin;

/// Following the NgRx architecture: https://ngrx.io/guide/store
///
///   Binding <--------------Store      Database
///     |                      ^           ^
///     |                      |           |
///     |                   Reducer     Service
///     |                      ^           ^
///     v                      |           |
///   View----> Facade----> Action <---> Effect
///
///
///
/// The Plugin's role is to allow the Bevy engine to bootstrap the component.
///
/// The View is effectively the DOM and Stylesheet of the UI widget.
///
/// The Facade dispatches actions on behalf of the UI, to update the Store.
///
/// The Actions' role is to invoke operations on the store.
///
/// The Reducer's job is to receive Actions and mutate the Store.
///
/// The Store is the core state of the UI Component. Acts as a "local database".
///
/// The Binding updates the View based on the Store.
///
/// The Effect queries or updates the database so that the Reducer can populate the Store with
///     a more authoritative, non-cached state as a side-effect of the given Action.
///
/// The Service pulls data from and pushes data to the source of truth.
///
/// The Database is the authoritative data layer for everything in general.
///
/// ISSUES:
/// - As a developer, I should be able to have multiple instances of this component: that is,
///     the component should not assume that it is a singleton. What I need is some way to "scope"
///     this thing, so that its Store, Events, and Components are local only to this instance.

mod plugin;

mod actions;
mod bindings;
mod facade;
mod reducer;
mod store;
mod view;