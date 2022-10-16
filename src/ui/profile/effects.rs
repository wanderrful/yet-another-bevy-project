use bevy::ecs::event::EventReader;
use bevy::ecs::system::Commands;
use bevy::tasks::AsyncComputeTaskPool;

use crate::ui::profile::actions;
use crate::ui::profile::services;

/// Call the GetIp service, whenever a ProfileActionGetIp event is broadcasted
pub fn get_ip(
    mut commands: Commands,
    mut get_ip_action: EventReader<actions::ProfileActionGetIp>
) {
    get_ip_action.iter().for_each(|_| {
        commands.spawn().insert(services::get_ip_task());
    });
}
