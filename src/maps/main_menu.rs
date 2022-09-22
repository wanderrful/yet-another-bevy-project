use bevy::app::{App, Plugin};
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::log::warn;

use crate::core::level_manager::{EnterLevel, LevelName};

pub struct MainMenuScenePlugin;

impl Plugin for MainMenuScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_assets);
    }
}

/// Initialize the main menu UI
fn spawn_assets(
    mut enter_level: EventReader<EnterLevel>,
) {
    enter_level.iter().for_each(|it| {
        match it.0 {
           LevelName::MainMenu => {
               warn!("spawn_assets::enter_level mainmenu");
           },
           _ => { }
        }
    });
}
