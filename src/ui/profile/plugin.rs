use std::collections::HashMap;

use bevy::app::{App, Plugin};
use bevy::ecs::event::EventWriter;
use bevy::ecs::system::Res;
use bevy::input::Input;
use bevy::input::keyboard::KeyCode;
use bevy::reflect::serde::MapSerializer;
use bevy::tasks::IoTaskPool;

use crate::services::httpbin::{GetIpResponse, HttpBinService};
use crate::services::reqres::ReqResService;
use crate::ui::profile::{actions, reducers, store};
use crate::ui::profile::effects;

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

            // TODO | Delete me!
            .add_system(temp)
            .add_system(temp2);
    }
}

/// Manually invoke actions for debugging purposes
fn temp(
    input: Res<Input<KeyCode>>,
    mut call_get_ip: EventWriter<actions::ProfileActionGetIp>,
    mut call_get_user_by_id: EventWriter<actions::ProfileActionGetUserById>
) {
    if input.just_pressed(KeyCode::F11) {
        call_get_ip.send(actions::ProfileActionGetIp);
    }
    if input.just_pressed(KeyCode::F10) {
        call_get_user_by_id.send(actions::ProfileActionGetUserById { id: 2 });
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
