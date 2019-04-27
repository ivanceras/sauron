//! https://developer.mozilla.org/en-US/docs/Web/Events
pub use sauron_vdom::builder::on;
use sauron_vdom::{builder::Attribute,
                  Callback,
                  Event};

macro_rules! declare_events {
    ( $(
         $(#[$attr:meta])*
         $name:ident => $event:ident;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $name<F, MSG>(f: F) -> Attribute<MSG>
                where F: Into<Callback<Event, MSG>>,
                      MSG: Clone
                {
                    on(stringify!($event), f)
                }
         )*
    }
}

// Mouse events
declare_events! {
    onauxclick => auxclick;
    onclick  => click;
    oncontextmenu =>contextmenu;
    ondblclick  => dblclick;
    onmousedown => mousedown;
    onmouseenter => mouseenter;
    onmouseleave => mouseleave;
    onmousemove => mousemove;
    onmouseover => mouseover;
    onmouseout => mouseout;
    onmouseup => mouseup;
    onpointerlockchange => pointerlockchange;
    onpointerlockerror => pointerlockerror;
    onselect => select;
    onwheel => wheel;
    ondoubleclick => doubleclick;
}

// keyboard events
declare_events! {
    onkeydown => keydown;
    onkeypress => keypress;
    onkeyup => keyup;
}

// focus events
declare_events! {
    onfocus => focus;
    onblur => blur;
}

// form events
declare_events! {
    onreset => reset;
    onsubmit => submit;
}

declare_events! {
    onbroadcast => broadcast;
    //CheckboxStateChange
    onhashchange => hashchange;
    oninput => input;
    //RadioStateChange
    onreadystatechange => readystatechange;
    //ValueChange
    onchange => change;
}
//TODO: add the rest of the events in the html specs
