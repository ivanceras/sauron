//! Create [events][0] Object
//!
//! [0]: https://developer.mozilla.org/en-US/docs/Web/Events
use crate::{
    html::attributes::AttributeValue,
    vdom::{Attribute, Listener},
};
use wasm_bindgen::JsCast;
#[cfg(web_sys_unstable_apis)]
pub use web_sys::ClipboardEvent;
pub use web_sys::{
    AnimationEvent, HashChangeEvent, KeyboardEvent, MouseEvent, TouchEvent,
    TransitionEvent,
};
use web_sys::{
    EventTarget, HtmlDetailsElement, HtmlInputElement, HtmlSelectElement,
    HtmlTextAreaElement,
};

/// Map the Event to DomEvent, which are browser events
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// native dome events web_sys::Events
    WebEvent(web_sys::Event),
    /// custom event here follows
    MountEvent(MountEvent),
}

impl Event {
    /// convert to web event
    pub fn as_web(self) -> Option<web_sys::Event> {
        match self {
            Event::WebEvent(web_event) => Some(web_event),
            _ => None,
        }
    }
}

impl From<MountEvent> for Event {
    fn from(mount_event: MountEvent) -> Self {
        Event::MountEvent(mount_event)
    }
}

impl From<web_sys::Event> for Event {
    fn from(web_event: web_sys::Event) -> Self {
        Event::WebEvent(web_event)
    }
}

impl From<web_sys::MouseEvent> for Event {
    fn from(mouse_event: MouseEvent) -> Self {
        let event: web_sys::Event = mouse_event
            .dyn_into()
            .expect("Unable to cast mouse event into event");
        Event::from(event)
    }
}

/// an event builder
pub fn on<F, MSG>(event_name: &'static str, f: F) -> Attribute<MSG>
where
    F: Fn(Event) -> MSG + 'static,
    MSG: 'static,
{
    mt_dom::attr(event_name, AttributeValue::EventListener(Listener::from(f)))
}

/// on click event
pub fn on_click<F, MSG>(f: F) -> Attribute<MSG>
where
    F: Fn(MouseEvent) -> MSG + 'static,
    MSG: 'static,
{
    on("click", move |event: Event| f(to_mouse_event(event)))
}

/// custom on_enter event, which is triggered from key_press when the Enter key is pressed
pub fn on_enter<F, MSG>(f: F) -> Attribute<MSG>
where
    F: Fn(KeyboardEvent) -> MSG + 'static,
    MSG: 'static,
{
    on("enter", move |event: Event| f(to_keyboard_event(event)))
}
/// attach callback to the scroll event
pub fn on_scroll<F, MSG>(f: F) -> Attribute<MSG>
where
    F: Fn((i32, i32)) -> MSG + 'static,
    MSG: 'static,
{
    on("scroll", move |event: Event| {
        let web_event = event.as_web().expect("must be a web event");
        let target = web_event.target().expect("can't get target");
        if let Some(element) = target.dyn_ref::<web_sys::Element>() {
            let scroll_top = element.scroll_top();
            let scroll_left = element.scroll_left();
            f((scroll_top, scroll_left))
        } else {
            let window = crate::window();
            let scroll_top =
                window.page_y_offset().expect("must get page offset") as i32;
            let scroll_left =
                window.page_x_offset().expect("must get page offset") as i32;
            f((scroll_top, scroll_left))
        }
    })
}

/// an event when a virtual Node is mounted the field node is the actual
/// dom node where the virtual Node is created in the actual dom
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MountEvent {
    /// the node where the virtual node is materialized into the actual dom
    pub target_node: web_sys::Node,
}

/// custom mount event
pub fn on_mount<F, MSG>(f: F) -> Attribute<MSG>
where
    F: Fn(MountEvent) -> MSG + 'static,
    MSG: 'static,
{
    on("mount", move |event: Event| match event {
        Event::MountEvent(me) => f(me),
        _ => {
            log::warn!("was expecting a mount event");
            panic!("not a mount event!")
        }
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
                pub fn $name<CB, MSG>(cb: CB) -> crate::vdom::Attribute<MSG>
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
        declare_events!{
            $(
                $(#[$attr])*
                $name => $event => $mapper => $ret;
            )*
        }

        /// html events
        pub const HTML_EVENTS: &[&'static str] = &[$(stringify!($event),)*];
    }
}

/// convert a generic event to MouseEvent
fn to_mouse_event(event: Event) -> MouseEvent {
    let web_event = event.as_web().expect("must be a web_sys event");
    web_event.dyn_into().expect("Unable to cast to mouse event")
}

fn to_keyboard_event(event: Event) -> KeyboardEvent {
    let web_event = event.as_web().expect("must be a web_sys event");
    web_event
        .dyn_into()
        .expect("unable to cast to keyboard event")
}

fn to_animation_event(event: Event) -> AnimationEvent {
    let web_event = event.as_web().expect("must be a web_sys event");
    web_event
        .dyn_into()
        .expect("unable to cast to animation event")
}

fn to_transition_event(event: Event) -> TransitionEvent {
    let web_event = event.as_web().expect("must be a web_sys event");
    web_event
        .dyn_into()
        .expect("unable to cast to transition event")
}

fn to_touch_event(event: Event) -> TouchEvent {
    let web_event = event.as_web().expect("must be web sys event");
    web_event.dyn_into().expect("unable to cast to touch event")
}

fn to_webevent(event: Event) -> web_sys::Event {
    match event {
        Event::WebEvent(event) => event,
        _ => panic!("not a web_event"),
    }
}

fn to_hashchange_event(event: Event) -> HashChangeEvent {
    let web_event = event.as_web().expect("must be a web_sys event");
    web_event
        .dyn_into()
        .expect("unable to cast to hashchange event")
}

/// a custom InputEvent to contain the input string value
#[derive(Debug)]
pub struct InputEvent {
    /// the input value
    pub value: String,
    /// the actual dom event
    pub event: web_sys::Event,
}

impl InputEvent {
    fn new(value: String, event: web_sys::Event) -> Self {
        InputEvent { value, event }
    }
}

fn to_input_event(event: Event) -> InputEvent {
    let web_event = event.as_web().expect("must be a web event");
    let target: EventTarget =
        web_event.target().expect("Unable to get event target");
    if let Some(input) = target.dyn_ref::<HtmlInputElement>() {
        InputEvent::new(input.value(), web_event)
    } else if let Some(textarea) = target.dyn_ref::<HtmlTextAreaElement>() {
        InputEvent::new(textarea.value(), web_event)
    } else if let Some(select) = target.dyn_ref::<HtmlSelectElement>() {
        InputEvent::new(select.value(), web_event)
    } else {
        panic!("fail in mapping event into input event");
    }
}

fn to_checked(event: Event) -> bool {
    let web_event = event.as_web().expect("must be a web event");
    let target: EventTarget =
        web_event.target().expect("Unable to get event target");
    if let Some(input) = target.dyn_ref::<HtmlInputElement>() {
        input.checked()
    } else {
        panic!("must be a html input element");
    }
}

fn to_open(event: Event) -> bool {
    let web_event = event.as_web().expect("must be a web event");
    let target: EventTarget =
        web_event.target().expect("Unable to get event target");
    if let Some(details) = target.dyn_ref::<HtmlDetailsElement>() {
        details.open()
    } else {
        panic!("must be a html input element");
    }
}

/// Note: paste event happens before the data is inserted into the target element
/// therefore trying to access the data on the target element triggered from paste will get an
/// empty text
#[cfg(web_sys_unstable_apis)]
fn to_clipboard_event(event: Event) -> ClipboardEvent {
    event
        .as_web()
        .expect("must be a web event")
        .dyn_into()
        .expect("unable to cast to clipboard event")
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
    on_popstate => popstate => to_webevent => web_sys::Event;
    on_select => select => to_webevent => web_sys::Event;
    on_wheel => wheel => to_mouse_event => MouseEvent;
    on_doubleclick => dblclick => to_mouse_event => MouseEvent;
    on_keydown => keydown => to_keyboard_event => KeyboardEvent;
    on_keypress => keypress => to_keyboard_event => KeyboardEvent;
    on_keyup => keyup => to_keyboard_event => KeyboardEvent;
    on_toggle => toggle => to_open => bool;
    on_touchstart => touchstart => to_touch_event => TouchEvent;
    on_touchend => touchend => to_touch_event => TouchEvent;
    on_touchmove => touchmove => to_touch_event => TouchEvent;
    on_focus => focus => to_webevent => web_sys::Event;
    on_blur => blur => to_webevent => web_sys::Event;
    on_reset => reset => to_webevent => web_sys::Event;
    on_submit => submit => to_webevent => web_sys::Event;
    on_input => input => to_input_event => InputEvent;
    on_checked => input => to_checked => bool;
    #[cfg(web_sys_unstable_apis)]
    on_paste => paste => to_clipboard_event => ClipboardEvent;
    #[cfg(web_sys_unstable_apis)]
    on_copy => copy => to_clipboard_event => ClipboardEvent;
    on_change => change => to_input_event => InputEvent;
    on_broadcast => broadcast => to_input_event => InputEvent;
    on_hashchange => hashchange => to_hashchange_event => HashChangeEvent;
    on_readystatechange => readystatechange => to_webevent => web_sys::Event;
}
