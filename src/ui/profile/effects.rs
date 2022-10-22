use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::system::Commands;
use bevy::tasks::AsyncComputeTaskPool;

use crate::services;
use crate::ui::profile::actions;

pub fn get_ip(
    mut commands: Commands,
    mut get_ip_action: EventReader<actions::ProfileActionGetIp>
) {
    get_ip_action.iter().for_each(|_| {
        commands.spawn().insert(services::httpbin::HttpBinService::get_ip());
    });
}

pub fn get_user(
    mut commands: Commands,
    mut get_user_action: EventReader<actions::ProfileActionGetUserById>
) {
    get_user_action.iter().for_each(|it| {
        let it: &actions::ProfileActionGetUserById = it;
        commands.spawn().insert(services::reqres::ReqResService::get_user_by_id(it.id));
    })
}
