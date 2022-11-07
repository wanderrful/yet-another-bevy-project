use std::cmp;

use bevy::ecs::event::EventReader;
use bevy::ecs::system::ResMut;

use crate::ui::text_input::{actions, store};

pub fn add_character(
    mut store: ResMut<store::Store>,
    mut add_content: EventReader<actions::ActionTextInputAddContent>,
) {
    add_content.iter().for_each(|it: &actions::ActionTextInputAddContent| {
        store.insert_character(&it.content);
    });
}

pub fn remove_character(
    mut store: ResMut<store::Store>,
    mut remove_content: EventReader<actions::ActionTextInputRemoveContent>
) {
    remove_content.iter().for_each(|it: &actions::ActionTextInputRemoveContent| {
        store.remove_character();
    });
}

pub fn move_cursor(
    mut store: ResMut<store::Store>,
    mut move_cursor: EventReader<actions::ActionTextInputMoveCursor>
) {
    move_cursor.iter().for_each(|it: &actions::ActionTextInputMoveCursor| {
        store.move_cursor(&it.direction);
    })
}
