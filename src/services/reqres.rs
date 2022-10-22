use bevy::app::{App, Plugin};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventWriter;
use bevy::ecs::system::{Commands, Query};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

pub struct ReqResService;

impl Plugin for ReqResService {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<GetUserResponse>()

            // Systems
            .add_system(ReqResService::process_response);
    }
}

impl ReqResService {
    pub fn get_user_by_id(id: u8) -> GetUserByIdTask {
        GetUserByIdTask {
            task: AsyncComputeTaskPool::get().spawn(async move {
                ehttp::fetch_blocking(&ehttp::Request::get(format!("https://reqres.in/api/users/{}", id)))
                    .expect("Error with HTTP call")
            })
        }
    }

    fn process_response(
        mut commands: Commands,
        mut tasks: Query<(Entity, &mut GetUserByIdTask)>,
        mut event: EventWriter<GetUserResponse>
    ) {
        tasks.iter_mut().for_each(|(e, mut t)| {
            if let Some(response) = future::block_on(future::poll_once(&mut t.task)) {
                // Process the response
                match serde_json::from_slice::<GetUserResponse>(response.bytes.as_slice()) {
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
pub struct GetUserByIdTask {
    pub task: Task<ehttp::Response>
}

#[derive(serde::Deserialize, Debug)]
pub struct GetUserResponse {
    pub data: GetUserResponseData,
    pub support: GetUserResponseSupport
}

#[derive(serde::Deserialize, Debug)]
pub struct GetUserResponseData {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar: String
}

#[derive(serde::Deserialize, Debug)]
pub struct GetUserResponseSupport {
    pub url: String,
    pub text: String
}
