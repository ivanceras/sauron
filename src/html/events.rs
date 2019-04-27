//! https://developer.mozilla.org/en-US/docs/Web/Events
pub use sauron_vdom::builder::{on,
                               on_with_mapper};
use sauron_vdom::Callback;

use extract::*;

pub mod extract {
    use wasm_bindgen::JsCast;
    use web_sys::{Event,
                  EventTarget,
                  HtmlInputElement,
                  HtmlTextAreaElement,
                  KeyboardEvent,
                  MouseEvent};

    /// extract the mouse x and y value regardless of what kind of mouse event
    pub fn mouse_event_extract_xy(event: Event) -> (i32, i32) {
        let mouse_event: Option<&MouseEvent> = event.dyn_ref();
        mouse_event.map(|mouse_event| (mouse_event.x(), mouse_event.y()))
                   .expect("Expecting a mouse event")
    }

    /// extract the input element value
    fn input_target_value(event: &Event) -> Option<String> {
        let target: EventTarget =
            event.target().expect("Expecting an event target");
        let input: Option<&HtmlInputElement> = target.dyn_ref();
        input.map(|input| input.value())
    }

    /// extract the value of the textarea element
    fn textarea_target_value(event: &Event) -> Option<String> {
        let target: EventTarget =
            event.target().expect("Expecting an event target");
        let textarea: Option<&HtmlTextAreaElement> = target.dyn_ref();
        textarea.map(|textarea| textarea.value())
    }

    pub fn generic_input_target_value(event: Event) -> String {
        if let Some(input_value) = input_target_value(&event) {
            input_value
        } else {
            textarea_target_value(&event)
                .expect("Expecting value from input or textarea element")
        }
    }

    /// extract the pressed key from a keyboard event
    pub fn keyboard_event_key(event: Event) -> String {
        let key_event: Option<&KeyboardEvent> = event.dyn_ref();
        key_event.map(|key_event| key_event.key())
                 .expect("Expecting a keyboard event")
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
    onauxclick : auxclick => |(i32, i32) | mouse_event_extract_xy;
    onclick : click => |(i32, i32)| mouse_event_extract_xy;
    oncontextmenu : contextmenu => |(i32, i32)| mouse_event_extract_xy ;
    ondblclick  : dblclick =>|(i32,i32) | mouse_event_extract_xy;
    onmousedown : mousedown =>|(i32,i32) | mouse_event_extract_xy;
    onmouseenter : mouseenter =>|(i32,i32) | mouse_event_extract_xy;
    onmouseleave : mouseleave =>|(i32,i32) | mouse_event_extract_xy;
    onmousemove : mousemove =>|(i32,i32) | mouse_event_extract_xy;
    onmouseover : mouseover =>|(i32,i32) | mouse_event_extract_xy;
    onmouseout : mouseout =>|(i32,i32) | mouse_event_extract_xy;
    onmouseup : mouseup =>|(i32,i32) | mouse_event_extract_xy;
    onpointerlockchange : pointerlockchange =>|(i32,i32) | mouse_event_extract_xy;
    onpointerlockerror : pointerlockerror =>|(i32,i32) | mouse_event_extract_xy;
    onselect : select =>|(i32,i32) | mouse_event_extract_xy;
    onwheel : wheel =>|(i32,i32) | mouse_event_extract_xy;
    ondoubleclick : doubleclick =>|(i32,i32) | mouse_event_extract_xy;
}

// keyboard events
declare_events! {
    onkeydown : keydown => |String| generic_input_target_value;
    onkeypress : keypress => |String| generic_input_target_value;
    onkeyup : keyup => |String | generic_input_target_value;
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
    oninput : input => |String| generic_input_target_value;
    onchange : change => | String | generic_input_target_value;
}
declare_events! {
    onbroadcast : broadcast;
    //CheckboxStateChange
    onhashchange : hashchange;
    //RadioStateChange
    onreadystatechange : readystatechange;
    //ValueChange
}
//TODO: add the rest of the events in the html specs
