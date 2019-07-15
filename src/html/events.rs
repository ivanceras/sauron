//! https://developer.mozilla.org/en-US/docs/Web/Events
use mapper::*;
pub use sauron_vdom::{
    builder::{
        on,
        on_with_extractor,
    },
    event::{
        Coordinate,
        InputEvent,
        KeyEvent,
        Modifier,
        MouseEvent,
        MouseButton,
    },
    Callback,
};
use wasm_bindgen::JsCast;

pub mod mapper {

    use sauron_vdom::event::{
        Coordinate,
        InputEvent,
        KeyEvent,
        Modifier,
        MouseEvent,
        MouseButton,
    };
    use wasm_bindgen::JsCast;
    use web_sys::{
        EventTarget,
        HtmlInputElement,
        HtmlTextAreaElement,
    };

    pub fn mouse_event_mapper(event: crate::Event) -> MouseEvent {
        let mouse: &web_sys::MouseEvent =
            event.0.dyn_ref().expect("Unable to cast to mouse event");
        let coordinate = Coordinate {
            client_x: mouse.client_x(),
            client_y: mouse.client_y(),
            movement_x: mouse.movement_x(),
            movement_y: mouse.movement_y(),
            offset_x: mouse.offset_x(),
            offset_y: mouse.offset_y(),
            screen_x: mouse.screen_x(),
            screen_y: mouse.screen_y(),
            x: mouse.x(),
            y: mouse.y(),
        };
        let modifier = Modifier {
            alt_key: mouse.alt_key(),
            ctrl_key: mouse.ctrl_key(),
            meta_key: mouse.meta_key(),
            shift_key: mouse.shift_key(),
        };
        let buttons = match mouse.button(){
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Left,
            3 => MouseButton::WheelUp,
            4 => MouseButton::WheelDown,
            _ => Default::default(), // defaults to left
        };
        let r#type = match &*event.0.type_(){
            "click" => "click",
            "mouseup" => "mouseup",
            "mousedown" => "mousedown",
            _e => panic!("unhandled event type: {}", _e),
        };
        MouseEvent{
            r#type,
            coordinate,
            modifier,
            buttons
        }
    }

    pub fn keyboard_event_mapper(event: crate::Event) -> KeyEvent {
        let key_event: &web_sys::KeyboardEvent =
            event.0.dyn_ref().expect("Unable to cast as key event");
        let modifier = Modifier {
            alt_key: key_event.alt_key(),
            ctrl_key: key_event.ctrl_key(),
            meta_key: key_event.meta_key(),
            shift_key: key_event.shift_key(),
        };
        KeyEvent {
            key: key_event.key(),
            modifier,
            repeat: key_event.repeat(),
            location: key_event.location(),
        }
    }

    pub fn input_event_mapper(event: crate::Event) -> InputEvent {
        let target: EventTarget =
            event.0.target().expect("Unable to get event target");
        let input: Option<&HtmlInputElement> = target.dyn_ref();
        let textarea: Option<&HtmlTextAreaElement> = target.dyn_ref();
        let input_event = if input.is_some() {
            input.map(|input| {
                InputEvent {
                    value: input.value(),
                }
            })
        } else if textarea.is_some() {
            textarea.map(|textarea| {
                InputEvent {
                    value: textarea.value(),
                }
            })
        } else {
            None
        };

        input_event.expect(
            "Expecting an input event from input element or textarea element",
        )
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
            pub fn $name<CB, MSG>(cb: CB) -> crate::Attribute<MSG>
                where CB: Fn($ret)-> MSG +'static,
                      MSG: 'static,
                {
                    on_with_extractor(stringify!($event), $mapper, cb)
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
            pub fn $name<CB, MSG>(cb: CB) -> crate::Attribute<MSG>
                where CB: Fn(()) -> MSG + 'static,
                      MSG: 'static,
                {
                    on_with_extractor(stringify!($event), |_|{}, cb)
                }
         )*
    }
}

#[inline]
pub fn onscroll<CB, MSG>(cb: CB) -> crate::Attribute<MSG>
where
    CB: Fn((i32, i32)) -> MSG + 'static,
    MSG: 'static,
{
    let webevent_to_scroll_offset = |event: crate::Event| {
        let target = event.0.target().expect("can't get target");
        let element: &web_sys::Element =
            target.dyn_ref().expect("Cant cast to Element");
        let scroll_top = element.scroll_top();
        let scroll_left = element.scroll_left();
        (scroll_top, scroll_left)
    };
    on_with_extractor("scroll", webevent_to_scroll_offset, cb)
}

pub fn onresize<CB, MSG>(cb: CB) -> crate::Attribute<MSG>
where
    CB: Fn((i32, i32)) -> MSG + 'static,
    MSG: 'static,
{
    crate::log("resizing..");
    let target_size_fn = |event: crate::Event| {
        let target = event.0.target().expect("can't get target");
        let element: &web_sys::Element =
            target.dyn_ref().expect("Cant cast to Element");
        let target_width = element.client_width();
        let target_height = element.client_height();
        (target_width, target_height)
    };
    on_with_extractor("resize", target_size_fn, cb)
}

// Mouse events
declare_events! {
    onclick : click => |MouseEvent| mouse_event_mapper;
    onauxclick : auxclick => |MouseEvent | mouse_event_mapper;
    oncontextmenu : contextmenu => |MouseEvent| mouse_event_mapper ;
    ondblclick  : dblclick =>|MouseEvent | mouse_event_mapper;
    onmousedown : mousedown =>|MouseEvent | mouse_event_mapper;
    onmouseenter : mouseenter =>|MouseEvent | mouse_event_mapper;
    onmouseleave : mouseleave =>|MouseEvent | mouse_event_mapper;
    onmousemove : mousemove =>|MouseEvent | mouse_event_mapper;
    onmouseover : mouseover =>|MouseEvent | mouse_event_mapper;
    onmouseout : mouseout =>|MouseEvent | mouse_event_mapper;
    onmouseup : mouseup =>|MouseEvent | mouse_event_mapper;
    onpointerlockchange : pointerlockchange =>|MouseEvent | mouse_event_mapper;
    onpointerlockerror : pointerlockerror =>|MouseEvent | mouse_event_mapper;
    onselect : select =>|MouseEvent | mouse_event_mapper;
    onwheel : wheel =>|MouseEvent | mouse_event_mapper;
    ondoubleclick : doubleclick =>|MouseEvent | mouse_event_mapper;
}

// keyboard events
declare_events! {
    onkeydown : keydown =>|KeyEvent| keyboard_event_mapper;
    onkeypress : keypress =>|KeyEvent| keyboard_event_mapper;
    onkeyup : keyup =>|KeyEvent| keyboard_event_mapper;
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
    oninput : input => |InputEvent| input_event_mapper;
    onchange : change => | InputEvent | input_event_mapper;
}
declare_events! {
    onbroadcast : broadcast;
    //CheckboxStateChange
    onhashchange : hashchange;
    //RadioStateChange
    onreadystatechange : readystatechange;
    //ValueChange
}
