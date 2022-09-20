use bevy::ecs::change_detection::ResMut;
use bevy::ecs::event::EventReader;

use crate::ui::counter::actions::CounterActionIncrement;
use crate::ui::counter::store::CounterStore;

/// Modify the Store's value by the given amount
pub fn reduce_increment(
    mut increment: EventReader<CounterActionIncrement>,
    mut state: ResMut<CounterStore>
) {
    increment.iter().for_each(|it: &CounterActionIncrement| { state.increment(it.delta); });
}
