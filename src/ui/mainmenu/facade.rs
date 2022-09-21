use bevy::app::AppExit;
use bevy::ecs::change_detection::Mut;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::{Local, Query, ResMut};
use bevy::render::color::Color;
use bevy::ui::{Interaction, Style, UiColor};
use bevy::window::{WindowFocused, WindowId, Windows};

use crate::ui::mainmenu::view::{UIMainMenuAboutButtonMarker, UIMainMenuExitButtonMarker, UIMainMenuOptionsButtonMarker, UIMainMenuPlayButtonMarker};

pub fn handle_play_button(mut button: Query<
    (&Interaction, &mut UiColor),
    (With<UIMainMenuPlayButtonMarker>, Changed<Interaction>)
>) {
    button.iter_mut().for_each(|(interaction, mut ui_color)| {
        *ui_color = UiColor::from(get_button_color(interaction));
    });
}

pub fn handle_options_button(mut button: Query<
    (&Interaction, &mut UiColor),
    (With<UIMainMenuOptionsButtonMarker>, Changed<Interaction>)
>) {
    button.iter_mut().for_each(|(interaction, mut ui_color)| {
        *ui_color = UiColor::from(get_button_color(interaction));
    });
}

pub fn handle_exit_button(mut button: Query<
        (&Interaction, &mut UiColor),
        (With<UIMainMenuExitButtonMarker>, Changed<Interaction>)
    >,
    mut exit: EventWriter<AppExit>
) {
    button.iter_mut().for_each(|(interaction, mut ui_color)| {
        *ui_color = UiColor::from(get_button_color(interaction));

        if Interaction::Clicked.eq(interaction) {
            exit.send(AppExit);
        }
    });
}

pub fn handle_about_button(mut button: Query<
    (&Interaction, &mut UiColor),
    (With<UIMainMenuAboutButtonMarker>, Changed<Interaction>)
>) {
    button.iter_mut().for_each(|(interaction, mut ui_color)| {
        *ui_color = UiColor::from(get_button_color(interaction));
    });
}

fn get_button_color(interaction: &Interaction) -> Color {
    match interaction {
        Interaction::Clicked => Color::LIME_GREEN,
        Interaction::Hovered => Color::DARK_GREEN,
        _ => Color::GRAY
    }
}