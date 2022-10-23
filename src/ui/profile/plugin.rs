use std::collections::HashMap;

use bevy::app::{App, Plugin};
use bevy::ecs::event::EventWriter;
use bevy::ecs::system::Res;
use bevy::input::Input;
use bevy::input::keyboard::KeyCode;
use bevy::reflect::serde::MapSerializer;
use bevy::tasks::IoTaskPool;
use rand::Rng;

use crate::services::httpbin::HttpBinService;
use crate::services::reqres::ReqResService;
use crate::ui::profile::{actions, bindings, effects, facades, reducers, store, view};

/// Plugin that bootstraps the Profile UI Menu and its dependencies.
pub struct ProfileMenuPlugin;

impl Plugin for ProfileMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Store
            .insert_resource(store::ProfileStore::default())

            // Actions
            .add_event::<actions::ProfileActionGetIp>()
            .add_event::<actions::ProfileActionGetUserById>()
            .add_event::<actions::ProfileActionMenuSetVisibility>()

            // Reducers
            .add_system(reducers::receive_ip)
            .add_system(reducers::receive_user_details)

            // Effects
            .add_system(effects::get_ip)
            .add_system(effects::get_user)

            // Services
            // TODO | These should either be Singletons, or they should be disposable!
            .add_plugin(HttpBinService)
            .add_plugin(ReqResService)

            // Views
            .add_startup_system(view::init_ui)

            // Bindings
            .add_system(bindings::bind_ip)
            .add_system(bindings::bind_user_id)
            .add_system(bindings::bind_user_email)
            .add_system(bindings::bind_user_name)

            // Facades
            .add_system(facades::handle_refresh_ip)
            .add_system(facades::handle_refresh_user_details)
            .add_system(facades::handle_set_visibility)

            // TODO | Delete me!
            .add_system(temp)
            .add_system(temp2);
    }
}

/// Manually invoke actions for debugging purposes
fn temp(
    input: Res<Input<KeyCode>>,
    mut set_profile_visibility: EventWriter<actions::ProfileActionMenuSetVisibility>
) {
    if input.just_pressed(KeyCode::LBracket) {
        set_profile_visibility.send(actions::ProfileActionMenuSetVisibility { visible: true });
    }
    if input.just_pressed(KeyCode::RBracket) {
        set_profile_visibility.send(actions::ProfileActionMenuSetVisibility { visible: false });
    }
}

/// Render the Store, whenever it mutates
fn temp2(
    store: Res<store::ProfileStore>
) {
    if store.is_changed() {
        bevy::log::info!("*** CHANGED store={:?}", store);
    }
}
