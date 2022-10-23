use bevy::ecs::change_detection::ResMut;
use bevy::ecs::event::EventReader;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::render::view::Visibility;
use bevy::ui::{Display, Style};

use crate::ui::counter::actions::CounterActionIncrement;
use crate::ui::counter::CounterActionMenuSetVisibility;
use crate::ui::counter::store::CounterStore;
use crate::ui::counter::view::UICounterParentMarker;

/// Modify the Store's value by the given amount
pub fn reduce_increment(
    mut increment: EventReader<CounterActionIncrement>,
    mut state: ResMut<CounterStore>
) {
    increment.iter().for_each(|it: &CounterActionIncrement| { state.increment(it.delta); });
}


/// Show or hide the entire Widget (related to the View, not to the Store)
pub fn reduce_visibility(
    mut event: EventReader<CounterActionMenuSetVisibility>,
    mut query: Query<&mut Style, With<UICounterParentMarker>>
) {
    event.iter().for_each(|it: &CounterActionMenuSetVisibility| {
        query.iter_mut()
            .for_each(|mut style| {
                style.display = if it.visible { Display::Flex } else { Display::None };
            });
    });
}
