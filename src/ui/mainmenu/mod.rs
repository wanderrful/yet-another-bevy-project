pub use plugin::MainMenuPlugin;

/// The MainMenu should have four buttons: Play, Options, About, and Exit:
///     - Clicking Play should remove the MainMenu and "load" the game.
///     - Clicking Options should hide the MainMenu and show the OptionsMenu.
///     - Clicking About should show the About panel as a pop-up.
///     - Clicking Exit should Exit the game.

mod plugin;

mod actions;
mod bindings;
mod facade;
mod reducer;
mod store;
mod view;

