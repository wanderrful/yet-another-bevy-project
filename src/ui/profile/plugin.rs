use std::collections::HashMap;

use bevy::app::{App, Plugin};
use bevy::log::info;
use bevy::reflect::serde::MapSerializer;

pub struct ProfileMenuPlugin;

impl Plugin for ProfileMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup);
    }
}

fn setup() {
    // GET https://httpbin.org/ip -> HashMap<String, String>
    ehttp::fetch(ehttp::Request::get("https://httpbin.org/ip"), move |res| {
        res.and_then(|it| {
            info!("headers={:?}", it.headers);
            info!("body={:?}", it.bytes);

            let deserialized: HashMap<String, String> = serde_json::from_slice(it.bytes.as_slice())
                .expect("Error occured while serializing fetched data. Is it proper JSON?");

            info!("deserialized={:?}", deserialized);

            if it.ok { Ok(()) } else { Err("error occurred".into()) }
        });
    });
}
