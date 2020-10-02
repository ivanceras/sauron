//! https://developer.mozilla.org/en-US/docs/Web/Events

use crate::{
    Attribute,
    Callback,
    Event,
};
use wasm_bindgen::JsCast;
pub use web_sys::{
    AnimationEvent,
    HashChangeEvent,
    KeyboardEvent,
    MouseEvent,
    TransitionEvent,
};
use web_sys::{
    EventTarget,
    HtmlInputElement,
    HtmlTextAreaElement,
};

/// an event builder
pub fn on<F, MSG>(event_name: &'static str, f: F) -> Attribute<MSG>
where
    F: Fn(Event) -> MSG + 'static,
{
    mt_dom::on(event_name, Callback::from(f))
}

/// on click event
pub fn on_click<F, MSG>(f: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
{
    on("click", move |event: Event| f(to_mouse_event(event)))
}
/// attach callback to the scroll event
pub fn on_scroll<F, MSG>(f: F) -> Attribute<MSG>
where
    F: Fn((i32, i32)) -> MSG + 'static,
    MSG: 'static,
{
    on("scroll", move |event: Event| {
        let target = event.target().expect("can't get target");
        let element: &web_sys::Element =
            target.dyn_ref().expect("Cant cast to Element");
        let scroll_top = element.scroll_top();
        let scroll_left = element.scroll_left();
        f((scroll_top, scroll_left))
    })
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
                        on(stringify!($event), move|event:Event|{
                            cb($mapper(event))
                        })
                }
            }
         )*
    }
}

macro_rules! declare_html_events{
    ( $(
         $(#[$attr:meta])*
         $name:ident => $event:ident => $mapper:ident => $ret:ty;
       )*
     ) => {
        declare_events!{ $($name => $event => $mapper => $ret;)* }

        /// html events
        pub const HTML_EVENTS: [&'static str; 30] = [$(stringify!($event),)*];
    }
}

/// convert a generic event to MouseEvent
fn to_mouse_event(event: Event) -> MouseEvent {
    event.dyn_into().expect("Unable to cast to mouse event")
}

fn to_keyboard_event(event: Event) -> KeyboardEvent {
    event.dyn_into().expect("unable to cast to keyboard event")
}

fn to_animation_event(event: Event) -> AnimationEvent {
    event.dyn_into().expect("unable to cast to animation event")
}

fn to_transition_event(event: Event) -> TransitionEvent {
    event
        .dyn_into()
        .expect("unable to cast to transition event")
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
declare_html_events! {
    on_auxclick => auxclick => to_mouse_event => MouseEvent;
    on_animationend => animationend => to_animation_event => AnimationEvent;
    on_transitionend => transitionend => to_transition_event => TransitionEvent;
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
    on_paste => paste => to_input_event => InputEvent;
    on_change => change => to_input_event => InputEvent;
    on_broadcast => broadcast => to_input_event => InputEvent;
    on_hashchange => hashchange => to_hashchange_event => HashChangeEvent;
    on_readystatechange => readystatechange => as_is => Event;
}
