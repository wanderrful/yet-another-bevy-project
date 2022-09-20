use bevy::ecs::event::EventWriter;
use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::Query;
use bevy::ui::Interaction;

use crate::ui::counter::actions::CounterActionIncrement;
use crate::ui::counter::view::{CounterActionDecreaseButtonMarker, CounterActionIncreaseButtonMarker};

/// Send Increment action when the Increment button is clicked
pub fn handle_interaction_increment(
    mut button: Query<
        &Interaction,
        (With<CounterActionIncreaseButtonMarker>, Changed<Interaction>)
    >,
    mut action: EventWriter<CounterActionIncrement>
) {
    button.get_single().iter()
        .filter(|it| Interaction::Clicked.eq(it))
        .inspect(|_| bevy::log::info!("ACTION SEND: '{}'", std::any::type_name::<CounterActionIncrement>()))
        .for_each(|_| action.send(CounterActionIncrement { delta: 1 }));
}

/// Send Decrement action when the Decrement button is clicked
pub fn handle_interaction_decrement(
    mut button: Query<
        &Interaction,
        (With<CounterActionDecreaseButtonMarker>, Changed<Interaction>)
    >,
    mut action: EventWriter<CounterActionIncrement>
) {
    button.get_single().iter()
        .filter(|it| Interaction::Clicked.eq(it))
        .inspect(|_| bevy::log::info!("ACTION SEND: '{}'", std::any::type_name::<CounterActionIncrement>()))
        .for_each(|it| action.send(CounterActionIncrement { delta: -1 }));
}