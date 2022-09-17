/// Following the NgRx architecture: https://dev.to/jagadeeshmusali/angular-ngrx-state-management-lifecycle-simplified-khl
///
///                 /--------Store
///                /           ^
///               /            |
/// Template<----/          Reducer
///     ^  \                  ^
///     |   \                 |
///    UI    \------------->Action
///

pub mod plugin;

mod actions;
mod facade;
mod reducer;
mod store;
mod template;
mod ui;