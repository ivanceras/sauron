/// A container for generic event and the common values
/// needed for the user.
/// This events are derived from their corresponding backend source
/// ie: html events from mouse, keypresses and input changes.
/// This events should also be recreatable from gtk-rs, libui-rs,
/// orbtk, ncurses, etc.
#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    MouseEvent(MouseEvent),
    KeyEvent(KeyEvent),
    InputEvent(InputEvent),
    Generic(String),
    Tick,
}

/// A mouse related event.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseEvent {
    /// A mouse button was pressed.
    ///
    /// The coordinates are one-based.
    Press(MouseButton, u16, u16),
    /// A mouse button was released.
    ///
    /// The coordinates are one-based.
    Release(u16, u16),
    /// A mouse button is held over the given coordinates.
    ///
    /// The coordinates are one-based.
    Hold(u16, u16),
}

/// A mouse button.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
    /// Mouse wheel is going up.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelUp,
    /// Mouse wheel is going down.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelDown,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct KeyEvent {
    pub key: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
}
impl KeyEvent {
    pub fn new(ch: char) -> Self {
        KeyEvent { key: ch.to_string(),
                   ..Default::default() }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputEvent {
    pub value: String,
}
