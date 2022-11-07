use bevy::ecs::event::EventWriter;
use bevy::ecs::system::Res;
use bevy::input::Input;
use bevy::input::keyboard::KeyCode;

use crate::ui::text_input::{actions, store};

pub fn handle_key_input(
    key_input: Res<Input<KeyCode>>,
    mut add_content: EventWriter<actions::ActionTextInputAddContent>,
    mut remove_content: EventWriter<actions::ActionTextInputRemoveContent>,
    mut move_cursor: EventWriter<actions::ActionTextInputMoveCursor>,
) {
    key_input.get_just_pressed().for_each(|it: &KeyCode| {
        if let Some(content) = map_keycode_to_char(it) {
            add_content.send(actions::ActionTextInputAddContent { content });
        }
        if map_keycode_to_remove(it) {
            remove_content.send(actions::ActionTextInputRemoveContent);
        }
        if let Some(direction) = map_keycode_to_arrow(it) {
            move_cursor.send(actions::ActionTextInputMoveCursor { direction })
        }
    });
}

// Determine whether this was a text character button
// TODO | There _must_ be a better way to deserialize the enum into a char...
fn map_keycode_to_char(it: &KeyCode) -> Option<char> {
    match it {
        // If this is a letter
        it if *it >= KeyCode::A && *it <= KeyCode::Z => {
            match it {
                KeyCode::A => Some('a'),
                KeyCode::B => Some('b'),
                KeyCode::C => Some('c'),
                KeyCode::D => Some('d'),
                KeyCode::E => Some('e'),
                KeyCode::F => Some('f'),
                KeyCode::G => Some('g'),
                KeyCode::H => Some('h'),
                KeyCode::I => Some('i'),
                KeyCode::J => Some('j'),
                KeyCode::K => Some('k'),
                KeyCode::L => Some('l'),
                KeyCode::M => Some('m'),
                KeyCode::N => Some('n'),
                KeyCode::O => Some('o'),
                KeyCode::P => Some('p'),
                KeyCode::Q => Some('q'),
                KeyCode::R => Some('r'),
                KeyCode::S => Some('s'),
                KeyCode::T => Some('t'),
                KeyCode::U => Some('u'),
                KeyCode::V => Some('v'),
                KeyCode::W => Some('w'),
                KeyCode::X => Some('x'),
                KeyCode::Y => Some('y'),
                KeyCode::Z => Some('z'),
                _ => { panic!("???"); }
            }
        },
        it if *it >= KeyCode::Key1 && *it <= KeyCode::Key0 => {
            match it {
                KeyCode::Key1 => Some('1'),
                KeyCode::Key2 => Some('2'),
                KeyCode::Key3 => Some('3'),
                KeyCode::Key4 => Some('4'),
                KeyCode::Key5 => Some('5'),
                KeyCode::Key6 => Some('6'),
                KeyCode::Key7 => Some('7'),
                KeyCode::Key8 => Some('8'),
                KeyCode::Key9 => Some('9'),
                KeyCode::Key0 => Some('0'),
                _ => { panic!("???"); }
            }
        }
        KeyCode::Space => Some(' '),
        _ => None
    }

}

// Determine whether this is the equivalent of a Backspace
fn map_keycode_to_remove(it: &KeyCode) -> bool {
    match it {
        KeyCode::Back => true,
        _ => false
    }
}

// Determine whether this was an cursor navigation button (e.g. arrow keys)
fn map_keycode_to_arrow(it: &KeyCode) -> Option<store::CursorMoveDirection> {
    match it {
        KeyCode::Up => Some(store::CursorMoveDirection::UP),
        KeyCode::Left => Some(store::CursorMoveDirection::LEFT),
        KeyCode::Down => Some(store::CursorMoveDirection::DOWN),
        KeyCode::Right => Some(store::CursorMoveDirection::RIGHT),
        _ => None
    }
}