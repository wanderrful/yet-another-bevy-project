use std::fmt::Debug;

use bevy::app::{App, Plugin};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::{Commands, Query};
use bevy::tasks::Task;
use futures_lite::future;
use serde::de::DeserializeOwned;
use serde::Deserialize;

/// Generic Component for making HTTP calls, so that we can consume responses via Query.
#[derive(Component)]
pub struct HttpCallTask {
    pub task: Task<ehttp::Response>
}

/// Poll, consume, and dispose of any existing HttpCallTasks.
pub fn consume_http_response(
    mut commands: Commands, mut tasks: Query<(Entity, &mut HttpCallTask)>
) {
    tasks.iter_mut().for_each(|(e, mut t)| {
        if let Some(response) = future::block_on(future::poll_once(&mut t.task)) {
            // TODO | How to decouple GetIpResponse from this? Templating doesn't seem to scale...
            match serde_json::from_slice::<crate::ui::profile::GetIpResponse>(response.bytes.as_slice()) {
                Ok(d) => {
                    bevy::log::info!("response={:?}", d);
                },
                Err(e) => {
                    bevy::log::error!("deserializationError={:?}", e);
                }
            }

            // Dispose of the consumed HTTP Call by deleting the Entity from ECS
            commands.entity(e).despawn();
        }
    });
}
