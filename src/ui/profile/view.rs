use bevy::asset::AssetServer;
use bevy::ecs::component::Component;
use bevy::ecs::system::{Commands, ResMut};
use bevy::hierarchy::BuildChildren;
use bevy::render::color::Color;
use bevy::text::{TextAlignment, TextStyle};
use bevy::ui::{AlignItems, AlignSelf, Display, FlexDirection, JustifyContent, Size, Style, UiColor, UiRect, Val};
use bevy::ui::entity::{ButtonBundle, NodeBundle, TextBundle};

use crate::ui::profile::bindings;

#[derive(Component)]
pub struct UIProfileParentMarker;

#[derive(Component)]
pub struct ProfileRefreshIPButtonMarker;

#[derive(Component)]
pub struct ProfileRefreshUserDetailsButtonMarker;


pub fn init_ui(mut commands: Commands, mut server: ResMut<AssetServer>) {
    ProfileComponent{}.init(commands, server);
}

struct ProfileComponent;

impl ProfileComponent {
    pub fn init(&self, mut commands: Commands, mut server: ResMut<AssetServer>) {
        commands
            .spawn_bundle(self.get_parent_component())
            .insert(UIProfileParentMarker)
            .with_children(|it| {
                it.spawn_bundle(self.get_ip_panel())
                    .with_children(|it2| {
                        it2.spawn_bundle(self.get_horizontal_box())
                            .with_children(|it3| {
                                it3.spawn_bundle(self.get_button(Color::GRAY.into()))
                                    .with_children(|it4| {
                                        it4.spawn_bundle(self.get_text_component(&server, "Refresh"));
                                    })
                                    .insert(ProfileRefreshIPButtonMarker);
                                it3.spawn_bundle(self.get_text_component(&server, "IP: "));
                                it3.spawn_bundle(self.get_text_component(&server, "X"))
                                    .insert(bindings::ProfileBindingIP);
                            });
                    });
                it.spawn_bundle(self.get_user_details_panel())
                    .with_children(|it2| {
                        it2.spawn_bundle(self.get_vertical_box())
                            .with_children(|it3| {
                                it3.spawn_bundle(self.get_horizontal_box())
                                    .with_children(|it4| {
                                        it4.spawn_bundle(self.get_button(Color::GRAY.into()))
                                            .with_children(|it5| {
                                                it5.spawn_bundle(self.get_text_component(&server, "Refresh"));
                                            })
                                            .insert(ProfileRefreshUserDetailsButtonMarker);
                                        it4.spawn_bundle(self.get_text_component(&server, "User Details: "));
                                    });
                                it3.spawn_bundle(self.get_vertical_box())
                                    .with_children(|it4| {
                                        it4.spawn_bundle(self.get_horizontal_box())
                                            .with_children(|it5| {
                                                it5.spawn_bundle(self.get_text_component(&server, "ID: "));
                                                it5.spawn_bundle(self.get_text_component(&server, "Y"))
                                                    .insert(bindings::ProfileBindingUserID);
                                            });
                                        it4.spawn_bundle(self.get_horizontal_box())
                                            .with_children(|it5| {
                                                it5.spawn_bundle(self.get_text_component(&server, "Email: "));
                                                it5.spawn_bundle(self.get_text_component(&server, "Z"))
                                                    .insert(bindings::ProfileBindingUserEmail);
                                            });
                                        it4.spawn_bundle(self.get_horizontal_box())
                                            .with_children(|it5| {
                                                it5.spawn_bundle(self.get_text_component(&server, "Name: "));
                                                it5.spawn_bundle(self.get_text_component(&server, "W"))
                                                    .insert(bindings::ProfileBindingUserName);
                                            });
                                    });
                            });
                    });
            });
    }

    fn get_parent_component(&self) -> NodeBundle {
        NodeBundle {
            color: Color::hsla(0.1, 0.1, 0.1, 0.3).into(),
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                display: Display::None,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_ip_panel(&self) -> NodeBundle {
        NodeBundle {
            color: Color::hsla(90.0, 0.4, 0.6, 0.4).into(),
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                margin: self.get_margin(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_user_details_panel(&self) -> NodeBundle {
        NodeBundle {
            color: Color::hsla(270.0, 0.4, 0.6, 0.4).into(),
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                margin: self.get_margin(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_text_component(&self, server: &ResMut<AssetServer>, text: &str) -> TextBundle {
        TextBundle::from_section(
            text,
            TextStyle {
                font: server.load("fonts/FiraSans-Bold.ttf"),
                font_size: self.get_font_size(),
                color: Color::WHITE,
            })
            .with_text_alignment(TextAlignment::CENTER)
            .with_style(Style {
                align_self: AlignSelf::Center,
                padding: self.get_margin(),
                ..Default::default()
            })
    }

    fn get_button(&self, color: UiColor) -> ButtonBundle {
        ButtonBundle {
            color,
            style: Style {
                size: Size::new(Val::Px(98.0), Val::Px(48.0)),
                margin: self.get_margin(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_vertical_box(&self) -> NodeBundle {
        NodeBundle {
            color: Color::hsla(0.1, 0.1, 0.1, 0.3).into(),
            style: Style {
                size: Size::new(Val::Auto, Val::Undefined),
                margin: self.get_margin(),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_horizontal_box(&self) -> NodeBundle {
        NodeBundle {
            color: Color::hsla(0.1, 0.1, 0.1, 0.3).into(),
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: self.get_margin(),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_margin(&self) -> UiRect<Val> {
        UiRect::new(
            Val::Px(6.0),
            Val::Px(6.0),
            Val::Px(2.0),
            Val::Px(2.0),
        )
    }

    fn get_font_size(&self) -> f32 {
        30.0
    }

}