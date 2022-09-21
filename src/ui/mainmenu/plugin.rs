use bevy::app::{App, Plugin};

use crate::ui::mainmenu::view::init_ui;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_ui);
    }
}
