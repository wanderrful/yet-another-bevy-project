use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::change_detection::{Mut, ResMut};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::{Commands, Query, Res};
use bevy::hierarchy::BuildChildren;
use bevy::input::Input;
use bevy::log::info;
use bevy::render::color::Color;
use bevy::text::{Text, TextAlignment, TextSection, TextStyle};
use bevy::ui::{Interaction, Size, Style, UiRect, Val};
use bevy::ui::entity::{ButtonBundle, NodeBundle, TextBundle};
use bevy::ui::widget::Button;

use crate::ui::counter::actions::{CounterActionIncrement, CounterActionMenuSetVisibility};
use crate::ui::counter::bindings::render_todo_text;
use crate::ui::counter::facade::{handle_interaction_decrement, handle_interaction_increment};
use crate::ui::counter::reducer::{reduce_increment, reduce_visibility};
use crate::ui::counter::store::CounterStore;
use crate::ui::counter::view::init_ui;

pub struct UICounterPlugin;


impl Plugin for UICounterPlugin {

    fn build(&self, app: &mut App) {
        app
            // (Store) the State of this UI widget
            .insert_resource(CounterStore::new(0))

            // (Actions) Declare Actions
            .add_event::<CounterActionIncrement>()
            .add_event::<CounterActionMenuSetVisibility>()

            // (Facade) Map interaction to Action
            .add_system(handle_interaction_increment)
            .add_system(handle_interaction_decrement)

            // (Reducers) Reduce Actions to changes in the Store
            .add_system(reduce_increment)
            .add_system(reduce_visibility)

            // (Bindings) Render front-end variables
            .add_system(render_todo_text)

            // Initialize the UI widget
            .add_startup_system(init_ui);
    }

}
