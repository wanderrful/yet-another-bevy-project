use bevy::ecs::component::Component;
use bevy::ecs::query::With;
use bevy::ecs::system::{Query, Res};
use bevy::text::Text;

use crate::ui::profile::bindings;
use crate::ui::profile::store;

#[derive(Component)]
pub struct ProfileBindingIP;

/// Populate the IP text field with its corresponding value in the Store
pub fn bind_ip(
    store: Res<store::ProfileStore>,
    mut render_text: Query<&mut Text, With<bindings::ProfileBindingIP>>,
) {
    if store.is_changed() {
        render_text.get_single_mut()
            .and_then(|mut it| Ok(it.sections[0].value = store.ip.to_string()))
            .unwrap();
    }
}


#[derive(Component)]
pub struct ProfileBindingUserID;

/// Populate the User ID text field with its corresponding value in the Store
pub fn bind_user_id(
    store: Res<store::ProfileStore>,
    mut render_text: Query<&mut Text, With<bindings::ProfileBindingUserID>>,
) {
    if store.is_changed() {
        render_text.get_single_mut()
            .and_then(|mut it| Ok(it.sections[0].value = store.user_id.to_string()))
            .unwrap();
    }
}


#[derive(Component)]
pub struct ProfileBindingUserEmail;

/// Populate the User Email text field with its corresponding value in the Store
pub fn bind_user_email(
    store: Res<store::ProfileStore>,
    mut render_text: Query<&mut Text, With<bindings::ProfileBindingUserEmail>>,
) {
    if store.is_changed() {
        render_text.get_single_mut()
            .and_then(|mut it| Ok(it.sections[0].value = store.email_address.to_string()))
            .unwrap();
    }
}


#[derive(Component)]
pub struct ProfileBindingUserName;

/// Populate the User Name text field with its corresponding values in the Store
pub fn bind_user_name(
    store: Res<store::ProfileStore>,
    mut render_text: Query<&mut Text, With<bindings::ProfileBindingUserName>>,
) {
    if store.is_changed() {
        render_text.get_single_mut()
            .and_then(|mut it| Ok(it.sections[0].value = format!("{} {}", store.first_name, store.last_name).to_string()))
            .unwrap();
    }
}
