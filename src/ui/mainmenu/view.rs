use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::ecs::change_detection::ResMut;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::hierarchy::BuildChildren;
use bevy::render::color::Color;
use bevy::text::{TextSection, TextStyle};
use bevy::ui::{AlignItems, AlignSelf, FlexDirection, JustifyContent, Size, Style, UiRect, Val};
use bevy::ui::entity::{ButtonBundle, NodeBundle, TextBundle};

#[derive(Component)]
pub struct UIMainMenuParentMarker;

#[derive(Component)]
pub struct UIMainMenuPlayButtonMarker;

#[derive(Component)]
pub struct UIMainMenuOptionsButtonMarker;

#[derive(Component)]
pub struct UIMainMenuExitButtonMarker;

#[derive(Component)]
pub struct UIMainMenuAboutButtonMarker;

/// Initialize the UI (Startup System)
pub fn init_ui(mut commands: Commands, mut server: ResMut<AssetServer>) {
    MainMenuComponent{}.init(commands, server);
}

struct MainMenuComponent;

impl MainMenuComponent {

    fn init(&self, mut commands: Commands, mut server: ResMut<AssetServer>) {
        commands
            .spawn_bundle(self.get_parent_component())
            .insert(UIMainMenuParentMarker)
            .with_children(|b0| {
                // The Game Title text
                b0.spawn_bundle(self.get_title_component(&server));

                // The Main Menu
                b0.spawn_bundle(self.get_vertical_box())
                    .with_children(|b1| {
                        b1.spawn_bundle(self.get_play_button())
                            .insert(UIMainMenuPlayButtonMarker)
                            .with_children(|b2| {
                                b2.spawn_bundle(self.get_text_component(&server, "Play"));
                            });
                        b1.spawn_bundle(self.get_options_button())
                            .insert(UIMainMenuOptionsButtonMarker)
                            .with_children(|b2| {
                                b2.spawn_bundle(self.get_text_component(&server, "Options"));
                            });
                        b1.spawn_bundle(self.get_exit_button())
                            .insert(UIMainMenuExitButtonMarker)
                            .with_children(|b2| {
                                b2.spawn_bundle(self.get_text_component(&server, "Exit"));
                            });
                    });

                // The About button
                b0.spawn_bundle(self.get_about_button())
                    .insert(UIMainMenuAboutButtonMarker)
                    .with_children(|b1| {
                        b1.spawn_bundle(self.get_text_component(&server, "About"));
                    });
            });
    }

    fn get_parent_component(&self) -> NodeBundle {
        NodeBundle {
            color: Color::hsla(0.0, 0.1, 0.1, 0.1).into(),
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_play_button(&self) -> ButtonBundle {
        self.get_button()
    }

    fn get_options_button(&self) -> ButtonBundle {
        self.get_button()
    }

    fn get_exit_button(&self) -> ButtonBundle {
        self.get_button()
    }

    fn get_about_button(&self) -> ButtonBundle {
        // Get the default button
        let mut out: ButtonBundle = self.get_button();

        // Configure it for this use case
        out.style = Style {
            align_self: AlignSelf::FlexEnd,
            margin: self.get_margin(),
            ..out.style
        };


        // Return it
        out
    }

    fn get_title_component(&self, server: &ResMut<AssetServer>) -> TextBundle {
        let mut out = self.get_text_component(&server, "GAME TITLE HERE");

        out.style = Style {
            margin: UiRect::new(Val::Auto, Val::Auto, Val::Percent(5.0), Val::Undefined),
            align_self: AlignSelf::Center,
            ..out.style
        };
        out.text.sections[0].style.font_size = 100.0;

        out
    }

    fn get_button(&self) -> ButtonBundle {
        ButtonBundle {
            color: Color::hsla(0.0, 0.0, 0.3, 0.6).into(),
            style: Style {
                margin: UiRect::all(Val::Px(12.0)),
                padding: self.get_margin(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_text_component(&self, server: &ResMut<AssetServer>, value: &str) -> TextBundle {
        TextBundle::from_section(
            value,
        TextStyle {
                font: server.load("fonts/FiraSans-Bold.ttf"),
                font_size: self.get_font_size(),
                color: Color::WHITE,
                ..Default::default()
            }
        )
    }

    fn get_vertical_box(&self) -> NodeBundle {
        NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                align_self: AlignSelf::FlexStart,
                size: Size::new(Val::Undefined, Val::Auto),
                margin: UiRect::new(Val::Percent(5.0), Val::Auto, Val::Auto ,Val::Auto),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_horizontal_box(&self) -> NodeBundle {
        NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_spacer(&self) -> NodeBundle {
        NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                size: Size::new(Val::Auto, Val::Percent(100.0)),
                flex_grow: 1.0,
                align_self: AlignSelf::Auto,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn get_font_size(&self) -> f32 { 60.0 }

    fn get_margin(&self) -> UiRect<Val> {
        UiRect::new(
            Val::Px(6.0),
            Val::Px(6.0),
            Val::Px(2.0),
            Val::Px(2.0),
        )
    }

}
