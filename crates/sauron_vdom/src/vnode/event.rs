/// A container for generic event and the common values
/// needed for the user.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    MouseEvent(MouseEvent),
    KeyEvent(KeyEvent),
    InputEvent(InputEvent),
}

/// A mouse event contains the (x,y) coordinates, buttons and modifier keys
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MouseEvent {
    coordinate: Coordinate,
    modifier: Modifier,
    buttons: Buttons,
}
impl MouseEvent {
    pub fn new(
        coordinate: Coordinate,
        modifier: Modifier,
        buttons: Buttons,
    ) -> Self {
        MouseEvent {
            coordinate,
            modifier,
            buttons,
        }
    }

    pub fn x(&self) -> i32 {
        self.coordinate.x()
    }

    pub fn y(&self) -> i32 {
        self.coordinate.y()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: String,
    pub modifier: Modifier,
    pub repeat: bool,
    pub location: u32,
}

impl KeyEvent {
    pub fn new(key: String) -> Self {
        KeyEvent {
            key,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct InputEvent {
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Buttons {
    pub button: i16,
    pub buttons: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Coordinate {
    pub client_x: i32,
    pub client_y: i32,
    pub movement_x: i32,
    pub movement_y: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub screen_x: i32,
    pub screen_y: i32,
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Modifier {
    pub alt_key: bool,
    pub ctrl_key: bool,
    pub meta_key: bool,
    pub shift_key: bool,
}
