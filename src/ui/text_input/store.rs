use std::cmp;

#[derive(Debug)]
pub struct Store {
    /// What the content of the Text Input should look like
    pub text_content: Vec<char>,

    /// Where the current focus is located in the text content
    pub cursor_position: (usize, usize),
}

impl Store {
    pub fn new() -> Self {
        Self {
            text_content: Vec::with_capacity(MAX_CONTENT_LENGTH),
            cursor_position: (0, 0)
        }
    }

    /// Insert a character at the cursor position, and move the cursor position forward.
    pub fn insert_character(&mut self, ch: &char) {
        let (x, y) = self.cursor_position;

        self.text_content.insert(x as usize, *ch);

        self.cursor_position = (
            cmp::min(x + 1, self.text_content.len()),
            y
        );
    }

    /// Remove the character immediately behind the cursor's position.
    pub fn remove_character(&mut self) {
        // No-op if there is no content to remove!
        if self.text_content.len() == 0 { return; }

        let (x, y) = self.cursor_position;

        // Remove the content at the current position
        self.text_content.remove(cmp::max(0, x - 1));

        // Move the cursor back
        self.cursor_position = (cmp::max(0, x - 1), y);
    }

    /// Move cursor position in the current direction.
    pub fn move_cursor(&mut self, direction: &CursorMoveDirection) {
        let (x, y) = self.cursor_position;

        self.cursor_position = match direction {
            CursorMoveDirection::UP => (x, if y > 0 { y - 1 } else { 0 }),
            CursorMoveDirection::LEFT => (if x > 0 { x - 1 } else { 0 }, y),
            CursorMoveDirection::DOWN => (x, cmp::min(MAX_CONTENT_LENGTH, y + 1)),
            CursorMoveDirection::RIGHT => (cmp::min(self.text_content.len(), x + 1), y),
        };
    }
}

pub enum CursorMoveDirection {
    UP,
    LEFT,
    DOWN,
    RIGHT
}

pub const MAX_CONTENT_LENGTH: usize = u8::MAX as usize;
