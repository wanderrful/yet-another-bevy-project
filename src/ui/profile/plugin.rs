use std::collections::HashMap;

use bevy::app::{App, Plugin};
use bevy::log::info;

pub struct ProfileMenuPlugin;

impl Plugin for ProfileMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup);
    }
}

fn setup() {
    reqwest::blocking::get("https://httpbin.org/ip")
        .and_then(|it| it.json::<HashMap<String, String>>())
        .and_then(|it| { info!("{:#?}", it); Ok(()) });
}
