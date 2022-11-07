use bevy::asset::AssetServer;
use bevy::ecs::system::{Commands, ResMut};
use bevy::hierarchy::BuildChildren;
use bevy::render::color::Color;
use bevy::text::{TextAlignment, TextStyle};
use bevy::ui::{AlignItems, AlignSelf, Display, FlexDirection, JustifyContent, Size, Style, UiRect, Val};
use bevy::ui::entity::{NodeBundle, TextBundle};

use crate::ui::text_input::bindings;

pub fn init_ui(mut commands: Commands, mut server: ResMut<AssetServer>) {
    TextInputView{}.init(commands, server);
}



struct TextInputView;

impl TextInputView {
    fn init(&self, mut commands: Commands, mut server: ResMut<AssetServer>) {
        commands
            .spawn_bundle(self.get_parent_component())
            .with_children(|it1| {
                it1.spawn_bundle(self.get_text_widget("***", &server))
                    .insert(bindings::TextInputBindingContent);
            })
        ;
    }

    fn get_parent_component(&self) -> NodeBundle {
        NodeBundle {
            color: Color::hsla(0.1, 0.1, 0.1, 0.3).into(),
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                display: Display::Flex,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_text_widget(&self, text: &str, server: &ResMut<AssetServer>) -> TextBundle {
        TextBundle::from_section(
            text,
            TextStyle {
                font: server.load("fonts/FiraSans-Bold.ttf"),
                font_size: self.get_font_size(),
                color: Color::WHITE,
            })
            .with_text_alignment(TextAlignment::TOP_LEFT)
            .with_style(Style {
                align_self: AlignSelf::FlexStart,
                padding: self.get_margin(),
                ..Default::default()
            })
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