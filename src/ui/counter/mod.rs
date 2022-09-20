/// The Bevy Plugin that orchestrates everything
pub use plugin::UICounterPlugin;

/// Following the NgRx architecture: https://dev.to/jagadeeshmusali/angular-ngrx-state-management-lifecycle-simplified-khl
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
/// ISSUES:
/// - As a developer, I should be able to have multiple instances of this component: that is,
///     the component should not assume that it is a singleton. What I need is some way to "scope"
///     this thing, so that its Store, Events, and Components are local only to this instance.



/// The Plugin's role is to allow the Bevy engine to bootstrap the component.
mod plugin;

/// The View is effectively the DOM and Stylesheet of the UI widget.
mod view;

/// The Facade dispatches actions to the Reducer.
mod facade;

/// The Actions' role is to invoke operations on the store.
mod actions;

/// The Reducer's job is to receive Actions and mutate the Store.
mod reducer;

/// The Store is the core state of the component. Acts as a "local database".
mod store;

/// The Binding updates the View based on the Store.
mod bindings;

/// The Effect queries or updates the database so that the Reducer can populate the Store with
///     a more authoritative, non-cached state as a side-effect of the given Action.

/// The Service pulls data from and pushes data to the source of truth.

/// The Database is the authoritative data layer for everything in general.