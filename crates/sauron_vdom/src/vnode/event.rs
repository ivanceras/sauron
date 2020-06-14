/// A container for generic event and the common values
/// needed for the user.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    MouseEvent(MouseEvent),
    KeyEvent(KeyEvent),
    InputEvent(InputEvent),
}

impl From<MouseEvent> for Event {
    fn from(me: MouseEvent) -> Self {
        Event::MouseEvent(me)
    }
}

impl From<InputEvent> for Event {
    fn from(ie: InputEvent) -> Self {
        Event::InputEvent(ie)
    }
}

impl From<KeyEvent> for Event {
    fn from(ke: KeyEvent) -> Self {
        Event::KeyEvent(ke)
    }
}

/// A mouse event contains the (x,y) coordinates, buttons and modifier keys
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MouseEvent {
    pub r#type: &'static str,
    pub coordinate: Coordinate,
    pub modifier: Modifier,
    pub buttons: MouseButton,
}
impl MouseEvent {
    pub fn click(x: i32, y: i32) -> Self {
        MouseEvent {
            r#type: "click",
            coordinate: Coordinate::new(x, y),
            //TODO: specify the buttons
            ..Default::default()
        }
    }

    pub fn pressed(x: i32, y: i32) -> Self {
        MouseEvent {
            r#type: "mousedown",
            coordinate: Coordinate::new(x, y),
            buttons: MouseButton::Left,
            ..Default::default()
        }
    }

    pub fn release(x: i32, y: i32) -> Self {
        MouseEvent {
            r#type: "mouseup",
            coordinate: Coordinate::new(x, y),
            buttons: MouseButton::Left,
            ..Default::default()
        }
    }

    pub fn mousemove(x: i32, y: i32) -> Self {
        MouseEvent {
            r#type: "mousemove",
            coordinate: Coordinate::new(x, y),
            buttons: MouseButton::Left,
            ..Default::default()
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

impl InputEvent {
    pub fn new(value: String) -> Self {
        InputEvent { value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    WheelUp,
    WheelDown,
}

impl Default for MouseButton {
    fn default() -> Self {
        MouseButton::Left
    }
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
    pub fn new(x: i32, y: i32) -> Self {
        Coordinate {
            x,
            y,
            ..Default::default()
        }
    }

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

impl Modifier {
    pub fn ctrl() -> Self {
        Modifier {
            ctrl_key: true,
            ..Default::default()
        }
    }
}
