use std::collections::HashMap;

use bevy::app::{App, Plugin};
use bevy::ecs::event::EventWriter;
use bevy::ecs::system::Res;
use bevy::input::Input;
use bevy::input::keyboard::KeyCode;
use bevy::reflect::serde::MapSerializer;
use bevy::tasks::IoTaskPool;

use crate::core::http::consume_http_response;
use crate::ui::profile::actions;
use crate::ui::profile::effects;
use crate::ui::profile::services::{get_user_by_id, GetIpResponse, GetUserResponse};

/// Plugin that bootstraps the Profile UI Menu and its dependencies.
pub struct ProfileMenuPlugin;

impl Plugin for ProfileMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<actions::ProfileActionGetIp>()
            .add_system(effects::get_ip)
            .add_system(consume_http_response)
            .add_system(temp)
            .add_startup_system(move || { get_user_by_id(2); });
    }
}

/// TODO | Delete me. Test that the "HTTP" flow works
fn temp(
    input: Res<Input<KeyCode>>,
    mut call_get_ip: EventWriter<actions::ProfileActionGetIp>
) {
    if input.just_pressed(KeyCode::F11) {
        call_get_ip.send(actions::ProfileActionGetIp);
    }
}
