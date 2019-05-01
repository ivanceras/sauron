//! https://developer.mozilla.org/en-US/docs/Web/Events
use mapper::*;
pub use sauron_vdom::builder::{on,
                               on_with_extractor};
use sauron_vdom::Callback;
use wasm_bindgen::JsCast;

pub mod mapper {

    use sauron_vdom::{event::Buttons,
                      Coordinate,
                      Modifier};
    use wasm_bindgen::JsCast;
    use web_sys::{EventTarget,
                  HtmlInputElement,
                  HtmlTextAreaElement};

    pub fn mouse_event_mapper(event: crate::Event)
                              -> sauron_vdom::MouseEvent {
        let mouse: &web_sys::MouseEvent =
            event.0.dyn_ref().expect("Unable to cast to mouse event");
        let coordinate = Coordinate { client_x: mouse.client_x(),
                                      client_y: mouse.client_y(),
                                      movement_x: mouse.movement_x(),
                                      movement_y: mouse.movement_y(),
                                      offset_x: mouse.offset_x(),
                                      offset_y: mouse.offset_y(),
                                      screen_x: mouse.screen_x(),
                                      screen_y: mouse.screen_y(),
                                      x: mouse.x(),
                                      y: mouse.y() };
        let modifier = Modifier { alt_key: mouse.alt_key(),
                                  ctrl_key: mouse.ctrl_key(),
                                  meta_key: mouse.meta_key(),
                                  shift_key: mouse.shift_key() };
        let buttons = Buttons { button: mouse.button(),
                                buttons: mouse.buttons() };
        sauron_vdom::MouseEvent::new(coordinate, modifier, buttons)
    }

    pub fn keyboard_event_mapper(event: crate::Event)
                                 -> sauron_vdom::KeyEvent {
        let key_event: &web_sys::KeyboardEvent =
            event.0.dyn_ref().expect("Unable to cast as key event");
        let modifier = Modifier { alt_key: key_event.alt_key(),
                                  ctrl_key: key_event.ctrl_key(),
                                  meta_key: key_event.meta_key(),
                                  shift_key: key_event.shift_key() };
        sauron_vdom::KeyEvent { key: key_event.key(),
                                modifier,
                                repeat: key_event.repeat(),
                                location: key_event.location() }
    }

    pub fn input_event_mapper(event: crate::Event)
                              -> sauron_vdom::InputEvent {
        let target: EventTarget =
            event.0.target().expect("Unable to get event target");
        let input: Option<&HtmlInputElement> = target.dyn_ref();
        let textarea: Option<&HtmlTextAreaElement> = target.dyn_ref();
        let input_event = if input.is_some() {
            input.map(|input| sauron_vdom::InputEvent { value: input.value() })
        } else if textarea.is_some() {
            textarea.map(|textarea| {
                        sauron_vdom::InputEvent { value: textarea.value() }
                    })
        } else {
            None
        };

        input_event.expect("Expecting an input event from input element or textarea element")
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
                      MSG: Clone + 'static,
                {
                    on_with_extractor(stringify!($event), |event|$mapper(event), cb)
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
                      MSG: Clone + 'static,
                {
                    on_with_extractor(stringify!($event), |_|{}, cb)
                }
         )*
    }
}

pub fn onscroll<CB, MSG>(cb: CB) -> crate::Attribute<MSG>
    where CB: Fn((i32, i32)) -> MSG + 'static,
          MSG: Clone + 'static
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
    oninput : input => |sauron_vdom::InputEvent| input_event_mapper;
    onchange : change => | sauron_vdom::InputEvent | input_event_mapper;
}
declare_events! {
    onbroadcast : broadcast;
    //CheckboxStateChange
    onhashchange : hashchange;
    //RadioStateChange
    onreadystatechange : readystatechange;
    //ValueChange
}
