use bevy::asset::AssetServer;
use bevy::ecs::component::Component;
use bevy::ecs::system::{Commands, ResMut};
use bevy::hierarchy::BuildChildren;
use bevy::render::color::Color;
use bevy::text::{TextAlignment, TextStyle};
use bevy::ui::{FlexDirection, Size, Style, UiColor, UiRect, Val};
use bevy::ui::entity::{ButtonBundle, NodeBundle, TextBundle};

use crate::ui::counter::actions::{CounterActionDecrement, CounterActionIncrement};
use crate::ui::counter::plugin::UICounter;
use crate::ui::counter::template::CounterStateRenderText;

/// Insert the UI widget.
pub fn init_ui(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>
) {
    CounterComponent{}.init(commands, asset_server);
}

struct CounterComponent;

impl CounterComponent {
    /// Initialize the CounterUI.
    pub fn init(&self, mut commands: Commands, mut server: ResMut<AssetServer>) {
        commands
            .spawn_bundle(self.get_parent_component())
            .insert(UICounter)
            .with_children(|builder| {
                builder
                    .spawn_bundle(self.get_vertical_box())
                    .with_children(|b| {
                        b.spawn_bundle(self.get_text_component(server)
                            .with_text_alignment(TextAlignment::TOP_CENTER))
                            .insert(CounterStateRenderText);
                    })
                    .with_children(|b| {
                        b.spawn_bundle(self.get_horizontal_box())
                            .with_children(|builder| {
                                builder.spawn_bundle(self.get_button(Color::GREEN.into()))
                                    .insert(CounterActionIncrement);
                                builder.spawn_bundle(self.get_button(Color::RED.into()))
                                    .insert(CounterActionDecrement);
                            });
                    });
            });
    }

    fn get_parent_component(&self) -> NodeBundle {
        NodeBundle {
            color: Color::hsla(0.1, 0.1, 0.1, 0.3).into(),
            style: Style {
                size: Size::new(Val::Percent(30.0), Val::Undefined),
                margin: self.get_margin(),
                flex_direction: FlexDirection::ColumnReverse,
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
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_horizontal_box(&self) -> NodeBundle {
        NodeBundle {
            color: Color::hsla(0.1, 0.1, 0.1, 0.3).into(),
            style: Style {
                size: Size::new(Val::Undefined, Val::Auto),
                margin: self.get_margin(),
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_text_component(&self, server: ResMut<AssetServer>) -> TextBundle {
        TextBundle::from_section(
            "DEFAULT TEXT",
            TextStyle {
                font: server.load("fonts/FiraSans-Bold.ttf"),
                font_size: self.get_font_size(),
                color: Color::WHITE,
            }
        )
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