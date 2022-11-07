use bevy::app::{App, Plugin};
use bevy::ecs::system::Res;

use crate::ui::text_input::{actions, bindings, facades, reducers, store, view};

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app
            // Store
            .insert_resource::<store::Store>(store::Store::new())

            // Actions
            .add_event::<actions::ActionTextInputAddContent>()
            .add_event::<actions::ActionTextInputRemoveContent>()
            .add_event::<actions::ActionTextInputMoveCursor>()

            // Bindings
            .add_system(bindings::bind_content)

            // Facades
            .add_system(facades::handle_key_input)

            // Reducers
            .add_system(reducers::add_character)
            .add_system(reducers::remove_character)
            .add_system(reducers::move_cursor)

            // View
            .add_startup_system(view::init_ui)

            // Debug
            .add_system(log_store)
        ;
    }
}

/// Log the content of the Store, for debugging purposes.
fn log_store(
    store: Res<store::Store>
) {
    if store.is_changed() {
        bevy::log::info!("store={:?}", store);
    }
}
