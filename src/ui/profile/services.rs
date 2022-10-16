use std::borrow::BorrowMut;
use std::error::Error;
use std::future::Future;
use std::sync::Arc;
use std::task::Poll;

use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::{Commands, Query};
use bevy::tasks::{AsyncComputeTaskPool, IoTaskPool, Task};
use futures_lite::future;

use crate::core::http::HttpCallTask;

/// Create the GetIP task, so that it will be polled and consumed by
///     the crate::core::http module's ECS.
pub fn get_ip_task() -> HttpCallTask {
    HttpCallTask {
        task: AsyncComputeTaskPool::get().spawn(async move {
            ehttp::fetch_blocking(&ehttp::Request::get("https://httpbin.org/ip"))
                .expect("Error with HTTP call")
        })
    }
}

/// Response DTO for the HTTP call to the Httpbin API.
#[derive(serde::Deserialize, Debug)]
pub struct GetIpResponse {
    pub origin: String
}



/// TODO | Using a second API call, to make sure that the design can scale.
pub fn get_user_by_id(id: i32) -> HttpCallTask {
    HttpCallTask {
        task: AsyncComputeTaskPool::get().spawn(async move {
            ehttp::fetch_blocking(&ehttp::Request::get(format!("https://reqres.in/api/users/{}", id)))
                .expect("Error with HTTP call")
        })
    }
}
// {
//     "data": {
//         "id": 2,
//         "email": "janet.weaver@reqres.in",
//         "first_name": "Janet",
//         "last_name": "Weaver",
//         "avatar": "https://reqres.in/img/faces/2-image.jpg"
//     },
//     "support": {
//         "url": "https://reqres.in/#support-heading",
//         "text": "To keep ReqRes free, contributions towards server costs are appreciated!"
//     }
// }
#[derive(serde::Deserialize, Debug)]
pub struct GetUserResponse {
    pub data: GetUserResponseData,
    pub support: GetUserResponseSupport
}

#[derive(serde::Deserialize, Debug)]
pub struct GetUserResponseData {
    id: i32,
    email: String,
    first_name: String,
    last_name: String,
    avatar: String
}

#[derive(serde::Deserialize, Debug)]
pub struct GetUserResponseSupport {
    url: String,
    text: String
}