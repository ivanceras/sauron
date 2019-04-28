//! https://developer.mozilla.org/en-US/docs/Web/Events
pub use sauron_vdom::builder::{on,
                               on_with_mapper};
use sauron_vdom::Callback;

use mapper::*;

pub mod mapper {

    /// extract the mouse x and y value regardless of what kind of mouse event
    pub fn mouse_event_mapper(event: sauron_vdom::Event)
                              -> sauron_vdom::MouseEvent {
        if let sauron_vdom::Event::MouseEvent(mouse_event) = event {
            mouse_event
        } else {
            panic!("Expecting a mouse event")
        }
    }

    /// extract the input element value
    pub fn input_value_mapper(event: sauron_vdom::Event)
                              -> sauron_vdom::InputEvent {
        if let sauron_vdom::Event::InputEvent(input_event) = event {
            input_event
        } else {
            panic!("Expecting an input event")
        }
    }

    /// extract the pressed key from a keyboard event
    pub fn keyboard_event_mapper(event: sauron_vdom::Event)
                                 -> sauron_vdom::KeyEvent {
        if let sauron_vdom::Event::KeyEvent(key_event) = event {
            key_event
        } else {
            panic!("Expecting a key event")
        }
    }
}

macro_rules! declare_events {
    ( $(
         $(#[$attr:meta])*
         $name:ident : $event:ident => | $ret:ty |  $mapper:ident;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $name<F, MSG>(f: F) -> crate::Attribute<MSG>
                where F: Into<Callback<$ret, MSG>>,
                      MSG: Clone + 'static,
                {
                    on_with_mapper(stringify!($event), |event|{$mapper(event)}, f)
                }
         )*
    };

    ( $(
         $(#[$attr:meta])*
         $name:ident : $event:ident;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $name<F, MSG>(f: F) -> crate::Attribute<MSG>
                where F: Into<Callback<(), MSG>>,
                      MSG: Clone + 'static,
                {
                    on_with_mapper(stringify!($event), |_|{}, f)
                }
         )*
    }
}

// Mouse events
declare_events! {
    onclick : click => |sauron_vdom::MouseEvent| mouse_event_mapper;
    onauxclick : auxclick => |sauron_vdom::MouseEvent | mouse_event_mapper;
    oncontextmenu : contextmenu => |sauron_vdom::MouseEvent| mouse_event_mapper ;
    ondblclick  : dblclick =>|sauron_vdom::MouseEvent | mouse_event_mapper;
    onmousedown : mousedown =>|sauron_vdom::MouseEvent | mouse_event_mapper;
    onmouseenter : mouseenter =>|sauron_vdom::MouseEvent | mouse_event_mapper;
    onmouseleave : mouseleave =>|sauron_vdom::MouseEvent | mouse_event_mapper;
    onmousemove : mousemove =>|sauron_vdom::MouseEvent | mouse_event_mapper;
    onmouseover : mouseover =>|sauron_vdom::MouseEvent | mouse_event_mapper;
    onmouseout : mouseout =>|sauron_vdom::MouseEvent | mouse_event_mapper;
    onmouseup : mouseup =>|sauron_vdom::MouseEvent | mouse_event_mapper;
    onpointerlockchange : pointerlockchange =>|sauron_vdom::MouseEvent | mouse_event_mapper;
    onpointerlockerror : pointerlockerror =>|sauron_vdom::MouseEvent | mouse_event_mapper;
    onselect : select =>|sauron_vdom::MouseEvent | mouse_event_mapper;
    onwheel : wheel =>|sauron_vdom::MouseEvent | mouse_event_mapper;
    ondoubleclick : doubleclick =>|sauron_vdom::MouseEvent | mouse_event_mapper;
}

// keyboard events
declare_events! {
    onkeydown : keydown =>|sauron_vdom::KeyEvent| keyboard_event_mapper;
    onkeypress : keypress =>|sauron_vdom::KeyEvent| keyboard_event_mapper;
    onkeyup : keyup =>|sauron_vdom::KeyEvent| keyboard_event_mapper;
}

// focus events
declare_events! {
    onfocus : focus;
    onblur : blur;
}

// form events
declare_events! {
    onreset : reset;
    onsubmit : submit;
}

declare_events! {
    oninput : input => |sauron_vdom::InputEvent| input_value_mapper;
    onchange : change => | sauron_vdom::InputEvent | input_value_mapper;
}
declare_events! {
    onbroadcast : broadcast;
    //CheckboxStateChange
    onhashchange : hashchange;
    //RadioStateChange
    onreadystatechange : readystatechange;
    //ValueChange
}
