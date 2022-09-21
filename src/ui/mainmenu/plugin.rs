use bevy::app::{App, Plugin};

use crate::ui::mainmenu::facade::{handle_about_button, handle_exit_button, handle_options_button, handle_play_button};
use crate::ui::mainmenu::reducer::reduce_visibility;
use crate::ui::mainmenu::view::init_ui;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Reducers
            .add_system(reduce_visibility)

            // Facades
            .add_system(handle_play_button)
            .add_system(handle_options_button)
            .add_system(handle_exit_button)
            .add_system(handle_about_button)

            // Initialize the UI
            .add_startup_system(init_ui);
    }
}
