use bevy::asset::AssetServer;
use bevy::ecs::component::Component;
use bevy::ecs::system::{Commands, ResMut};
use bevy::hierarchy::BuildChildren;
use bevy::render::color::Color;
use bevy::text::{TextAlignment, TextStyle};
use bevy::ui::{AlignItems, AlignSelf, Display, FlexDirection, JustifyContent, Size, Style, UiColor, UiRect, Val};
use bevy::ui::entity::{ButtonBundle, NodeBundle, TextBundle};

use crate::ui::counter::bindings::CounterStateRenderText;

/// Provide a reference to the parent component of this UI, for querying purposes
#[derive(Component)]
pub struct UICounterParentMarker;

/// Provide a reference to the "+" button
#[derive(Component)]
pub struct CounterActionIncreaseButtonMarker;

/// Provide a reference to the "-" button
#[derive(Component)]
pub struct CounterActionDecreaseButtonMarker;


/// Insert the UI widget.
pub fn init_ui(mut commands: Commands, mut server: ResMut<AssetServer>) {
    CounterComponent{}.init(commands, server);
}

struct CounterComponent;

impl CounterComponent {
    /// Initialize the CounterUI.
    pub fn init(&self, mut commands: Commands, mut server: ResMut<AssetServer>) {
        commands
            .spawn_bundle(self.get_parent_component())
            .insert(UICounterParentMarker)
            .with_children(|b0| {
                b0.spawn_bundle(self.get_vertical_box())
                    .with_children(|b1| {
                        b1.spawn_bundle(self.get_horizontal_box())
                            .with_children(|b2| {
                                b2.spawn_bundle(self.get_text_component(&server, "Value: "));
                                b2.spawn_bundle(self.get_text_component(&server, "X"))
                                    .insert(CounterStateRenderText);
                            });
                        b1.spawn_bundle(self.get_horizontal_box())
                            .with_children(|b2| {
                                b2.spawn_bundle(self.get_button(Color::GREEN.into()))
                                    .insert(CounterActionIncreaseButtonMarker)
                                    .with_children(|b3| {
                                        b3.spawn_bundle(self.get_text_component(&server, "+"));
                                    });
                                b2.spawn_bundle(self.get_button(Color::RED.into()))
                                    .insert(CounterActionDecreaseButtonMarker)
                                    .with_children(|b3| {
                                        b3.spawn_bundle(self.get_text_component(&server, "-"));
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
                margin: self.get_margin(),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                display: Display::None,
                ..Default::default()
            },
            ..Default::default()
        }
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

    fn get_font_size(&self) -> f32 {
        60.0
    }

    fn get_margin(&self) -> UiRect<Val> {
        UiRect::new(
            Val::Px(6.0),
            Val::Px(6.0),
            Val::Px(2.0),
            Val::Px(2.0),
        )
    }

}