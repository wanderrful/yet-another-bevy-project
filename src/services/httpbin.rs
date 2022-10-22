use bevy::app::{App, Plugin};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventWriter;
use bevy::ecs::system::{Commands, Query};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

/// Add this Service to another Plugin, to show dependency on API calls.
pub struct HttpBinService;

impl Plugin for HttpBinService {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<GetIpResponse>()

            // Systems
            .add_system(HttpBinService::process_response);
    }
}

impl HttpBinService {
    /// NOTE | Creating a Task does not begin running the task.
    pub fn get_ip() -> GetIpTask {
        GetIpTask {
            task: AsyncComputeTaskPool::get().spawn(async move {
                ehttp::fetch_blocking(&ehttp::Request::get("https://httpbin.org/ip"))
                    .expect("Error with HTTP call")
            })
        }
    }

    /// Monitor existing GetIp calls to process their responses as they complete. (ECS System)
    fn process_response(
        mut commands: Commands,
        mut tasks: Query<(Entity, &mut GetIpTask)>,
        mut event: EventWriter<GetIpResponse>
    ) {
        tasks.iter_mut().for_each(|(e, mut t)| {
            if let Some(response) = future::block_on(future::poll_once(&mut t.task)) {
                // Process the response
                match serde_json::from_slice::<GetIpResponse>(response.bytes.as_slice()) {
                    Ok(d) => {
                        event.send(d);
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
}

#[derive(Component)]
pub struct GetIpTask {
    pub task: Task<ehttp::Response>
}

#[derive(serde::Deserialize, Debug)]
pub struct GetIpResponse {
    pub origin: String
}
