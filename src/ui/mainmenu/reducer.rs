use bevy::ecs::event::EventReader;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::render::view::Visibility;

use crate::ui::mainmenu::actions::MainMenuActionMenuSetVisibility;
use crate::ui::mainmenu::view::UIMainMenuParentMarker;

/// Show or hide the entire Widget
pub fn reduce_visibility(
    mut event: EventReader<MainMenuActionMenuSetVisibility>,
    mut query: Query<&mut Visibility, With<UIMainMenuParentMarker>>
) {
    event.iter().for_each(|it: &MainMenuActionMenuSetVisibility| {
        query.iter_mut()
            .for_each(|mut visibility| {
                visibility.is_visible = it.visible;
            });
    });
}
