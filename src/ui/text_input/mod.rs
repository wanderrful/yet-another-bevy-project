pub use plugin::TextInputPlugin;

/// DESIGN for TEXT INPUT UI widget
///
/// MVP requirements (per https://github.com/bevyengine/bevy/issues/6213):
/// - Proper cursor position handling
/// - Standard behavior, like copy, paste, select with native keyboard shortcut handling
/// - Readonly toggling
/// - Show/hide value (for passwords)
/// - Placeholder text
/// - IME support
/// - LTR + RTL
/// - Text overflow
/// - A11y for stuff like screen readers
///
///
/// Store:
/// - text content
/// - cursor position, within the text content
///
/// Actions:
/// *** (should only apply when widget is focused)
/// - Mutate text content
/// - Mutate cursor position
///
/// Bindings:
/// - Render text content
/// - Render cursor position
///
/// View:
/// - Box containing text content
///
///
mod plugin;
mod actions;
mod bindings;
mod effects;
mod facades;
mod reducers;
mod store;
mod view;