use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::Query;
use bevy::ui::{Display, Interaction, Style};
use rand::Rng;

use crate::ui::profile::{actions, view};
use crate::ui::profile::actions::ProfileActionMenuSetVisibility;
use crate::ui::profile::view::UIProfileParentMarker;

/// Initiate HTTP call to HttpBin to resolve the current IP
pub fn handle_refresh_ip(
    mut button: Query<
        &Interaction,
        (With<view::ProfileRefreshIPButtonMarker>, Changed<Interaction>)
    >,
    mut action: EventWriter<actions::ProfileActionGetIp>
) {
    button.get_single().iter()
        .filter(|it| Interaction::Clicked.eq(it))
        .inspect(|_| bevy::log::debug!("ACTION SEND: '{}'", std::any::type_name::<actions::ProfileActionGetIp>()))
        .for_each(|_| action.send( actions::ProfileActionGetIp));
}

/// Initiate HTTP call to ReqRes to resolve User Details
pub fn handle_refresh_user_details(
    mut button: Query<
        &Interaction,
        (With<view::ProfileRefreshUserDetailsButtonMarker>, Changed<Interaction>)
    >,
    mut action: EventWriter<actions::ProfileActionGetUserById>
) {
    button.get_single().iter()
        .filter(|it| Interaction::Clicked.eq(it))
        .inspect(|_| bevy::log::debug!("ACTION SEND: '{}'", std::any::type_name::<actions::ProfileActionGetUserById>()))
        .for_each(|_| action.send( actions::ProfileActionGetUserById { id: rand::thread_rng().gen_range(1..13) }));
}

/// Show or hide the entire Widget
pub fn handle_set_visibility(
    mut event: EventReader<actions::ProfileActionMenuSetVisibility>,
    mut query: Query<&mut Style, With<UIProfileParentMarker>>
) {
    event.iter().for_each(|it: &ProfileActionMenuSetVisibility| {
        query.iter_mut()
            .for_each(|mut style| {
                style.display = if it.visible { Display::Flex } else { Display::None };
            });
    });
}