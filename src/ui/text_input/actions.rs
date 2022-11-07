use crate::ui::text_input::store;

/// When we should append a character at Store::cursor_position
pub struct ActionTextInputAddContent {
    pub content: char,
}

/// When we should remove a character at Store::cursor_position
pub struct ActionTextInputRemoveContent;

/// When we want to move the text cursor position
pub struct ActionTextInputMoveCursor {
    pub direction: store::CursorMoveDirection
}
