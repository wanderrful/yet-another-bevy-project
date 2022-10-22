use bevy::ecs::event::EventReader;
use bevy::ecs::system::ResMut;

use crate::services;
use crate::ui::profile::actions;
use crate::ui::profile::store;

pub fn receive_ip(
    mut store: ResMut<store::ProfileStore>,
    mut event: EventReader<services::httpbin::GetIpResponse>
) {
    event.iter().for_each(|it| {
        let it: &services::httpbin::GetIpResponse = it;

        store.ip = it.origin.to_string();
    });
}

pub fn receive_user_details(
    mut store: ResMut<store::ProfileStore>,
    mut event: EventReader<services::reqres::GetUserResponse>
) {
    event.iter().for_each(|it| {
        let it: &services::reqres::GetUserResponse = it;

        store.user_id = it.data.id;
        store.email_address = it.data.email.to_string();
        store.first_name = it.data.first_name.to_string();
        store.last_name = it.data.last_name.to_string();
    })
}