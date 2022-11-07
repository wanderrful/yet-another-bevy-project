use bevy::ecs::component::Component;
use bevy::ecs::query::With;
use bevy::ecs::system::{Query, Res};
use bevy::text::Text;

use crate::ui::text_input::store;

/// Marker corresponding to the content of the Text Input widget, for ECS querying purposes.
#[derive(Component)]
pub struct TextInputBindingContent;

pub fn bind_content(
    store: Res<store::Store>,
    mut content: Query<&mut Text, With<TextInputBindingContent>>,
) {
    if store.is_changed() {
        let mut it = content.get_single_mut().unwrap();

        let (x, y) = store.cursor_position;
        let mut new_text: String = store.text_content.iter().collect();

        // Insert cursor character
        new_text.insert(x, '|');

        // Set the widget's text
        it.sections[0].value = new_text;
    }
}
