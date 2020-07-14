//! https://developer.mozilla.org/en-US/docs/Web/Events

use crate::{
    prelude::Callback,
    Attribute,
    AttributeValue,
    Event,
};
use wasm_bindgen::JsCast;
use web_sys::{
    EventTarget,
    HtmlInputElement,
    HtmlTextAreaElement,
};

pub use web_sys::{
    HashChangeEvent,
    KeyboardEvent,
    MouseEvent,
};

/// on click event
pub fn on_click<F, MSG>(f: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    mt_dom::attr(
        "click",
        AttributeValue::from_callback(Callback::from(move |event: Event| {
            f(to_mouse_event(event))
        })),
    )
}
/// attach callback to the scroll event
pub fn on_scroll<F, MSG>(f: F) -> Attribute<MSG>
where
    F: Fn((i32, i32)) -> MSG + 'static,
    MSG: 'static,
{
    mt_dom::attr(
        "scroll",
        AttributeValue::from_callback(Callback::from(move |event: Event| {
            let target = event.target().expect("can't get target");
            let element: &web_sys::Element =
                target.dyn_ref().expect("Cant cast to Element");
            let scroll_top = element.scroll_top();
            let scroll_left = element.scroll_left();
            f((scroll_top, scroll_left))
        })),
    )
}

macro_rules! declare_events {

    ( $(
         $(#[$attr:meta])*
         $name:ident => $event:ident => $mapper:ident => $ret:ty;
       )*
     ) => {
        $(
            doc_comment!{
                concat!("attach an [",stringify!($name),"](https://developer.mozilla.org/en-US/docs/Web/API/GlobalEventHandlers/",stringify!($name),") event to the html element"),
                $(#[$attr])*
                #[inline]
                pub fn $name<CB, MSG>(cb: CB) -> crate::Attribute<MSG>
                    where CB: Fn($ret) -> MSG + 'static,
                          MSG: 'static,
                    {
                        mt_dom::attr(stringify!($event), AttributeValue::from_callback(Callback::from(move|event:Event|{
                            cb($mapper(event))
                        })))
                }
            }
         )*
    }
}

/// convert a generic event to MouseEvent
fn to_mouse_event(event: Event) -> MouseEvent {
    event.dyn_into().expect("Unable to cast to mouse event")
}

fn to_keyboard_event(event: Event) -> KeyboardEvent {
    event.dyn_into().expect("unable to cast to keyboard event")
}

fn as_is(event: Event) -> Event {
    event
}

fn to_hashchange_event(event: Event) -> HashChangeEvent {
    event
        .dyn_into()
        .expect("unable to cast to hashchange event")
}

/// a custom InputEvent to contain the input string value
pub struct InputEvent {
    /// the input value
    pub value: String,
}

impl InputEvent {
    fn new(value: String) -> Self {
        InputEvent { value }
    }
}

fn to_input_event(event: Event) -> InputEvent {
    let target: EventTarget =
        event.target().expect("Unable to get event target");
    if let Some(input) = target.dyn_ref::<HtmlInputElement>() {
        InputEvent::new(input.value())
    } else if let Some(textarea) = target.dyn_ref::<HtmlTextAreaElement>() {
        InputEvent::new(textarea.value())
    } else {
        panic!("fail in mapping event into input event");
    }
}

// Mouse events
declare_events! {
    on_auxclick => auxclick => to_mouse_event => MouseEvent;
    on_contextmenu => contextmenu => to_mouse_event => MouseEvent;
    on_dblclick  => dblclick => to_mouse_event => MouseEvent;
    on_mousedown => mousedown => to_mouse_event => MouseEvent;
    on_mouseenter => mouseenter => to_mouse_event => MouseEvent;
    on_mouseleave => mouseleave => to_mouse_event => MouseEvent;
    on_mousemove => mousemove => to_mouse_event => MouseEvent;
    on_mouseover => mouseover => to_mouse_event => MouseEvent;
    on_mouseout => mouseout => to_mouse_event => MouseEvent;
    on_mouseup => mouseup => to_mouse_event => MouseEvent;
    on_pointerlockchange => pointerlockchange => to_mouse_event => MouseEvent;
    on_pointerlockerror => pointerlockerror => to_mouse_event => MouseEvent;
    on_select => select => as_is => Event;
    on_wheel => wheel => to_mouse_event => MouseEvent;
    on_doubleclick => dblclick => to_mouse_event => MouseEvent;
    on_keydown => keydown => to_keyboard_event => KeyboardEvent;
    on_keypress => keypress => to_keyboard_event => KeyboardEvent;
    on_keyup => keyup => to_keyboard_event => KeyboardEvent;
    on_focus => focus => as_is => Event;
    on_blur => blur => as_is => Event;
    on_reset => reset => as_is => Event;
    on_submit => submit => as_is => Event;
    on_input => input => to_input_event => InputEvent;
    on_change => change => to_input_event => InputEvent;
    on_broadcast => broadcast => to_input_event => InputEvent;
    on_hashchange => hashchange => to_hashchange_event => HashChangeEvent;
    on_readystatechange => readystatechange => as_is => Event;
}
