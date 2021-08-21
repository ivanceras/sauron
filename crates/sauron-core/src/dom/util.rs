use wasm_bindgen::{closure::Closure, JsCast};

/// utility function which returns the Window element
pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

/// utility function which returns the history api of the browser
pub fn history() -> web_sys::History {
    window().history().expect("should have a history object")
}

/// utility function which executes the agument closure in a request animation frame
pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

/// provides access to the document element
pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

/// provides access to the document body
pub fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

/// provides access to the window Performance api
pub fn performance() -> web_sys::Performance {
    window()
        .performance()
        .expect("should have performance on window")
}

/// return the instantaneous time
pub fn now() -> f64 {
    performance().now()
}
