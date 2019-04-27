use wasm_bindgen::{closure::Closure,
                   JsCast};
use web_sys;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<FnMut()>) {
    window().request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
}

pub fn document() -> web_sys::Document {
    window().document()
            .expect("should have a document on window")
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

pub fn performance() -> web_sys::Performance {
    window().performance()
            .expect("should have performance on window")
}

#[inline]
pub fn log<S: Into<String>>(s: S) {
    web_sys::console::log_1(&s.into().into());
}

#[inline]
#[macro_export]
macro_rules! log {
    ($($t:tt)*) => ($crate::log(format!($($t)*)))
}
