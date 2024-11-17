//! utility functions
//!
use crate::dom;
pub use wasm_bindgen_futures::spawn_local;
use web_sys::ScrollToOptions;

//TODO: feature gate this with `use-cached-windows`
thread_local!(static WINDOW: web_sys::Window = web_sys::window().expect("no global `window` exists"));
thread_local!(static DOCUMENT: web_sys::Document = window().document().expect("should have a document on window"));

/// utility function which returns the Window element
pub fn window() -> web_sys::Window {
    WINDOW.with(|window| window.clone())
}

/// provides access to the document element
pub fn document() -> web_sys::Document {
    DOCUMENT.with(|document| document.clone())
}

/// utility function which returns the history api of the browser
pub fn history() -> web_sys::History {
    window().history().expect("should have a history object")
}

/// inject style to document head
pub fn inject_style(style: &str) {
    let head = document().head().expect("must have a head");
    let style_node = document()
        .create_element("style")
        .expect("create style element");
    let style_css = document().create_text_node(style);
    style_node
        .append_child(&style_css)
        .expect("append to style");
    head.append_child(&style_node).expect("must append to head");
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

/// scroll the browser to the top of the document
pub fn scroll_window_to_top() {
    let options = ScrollToOptions::new();
    options.set_top(0.0);
    options.set_left(0.0);
    window().scroll_to_with_scroll_to_options(&options);
}

/// set the browser location hash
pub fn set_location_hash(hash: &str) {
    let location = window().location();
    location.set_hash(hash).expect("must set the location hash");
}

/// return the hash part of the browser current url location
/// The hash part are the text right after the `#` sign
pub fn get_location_hash() -> String {
    window().location().hash().expect("must have a hash")
}

/// return the size of the browser at this moment
pub fn get_window_size() -> (i32, i32) {
    let window = dom::window();
    let window_width = window
        .inner_width()
        .expect("unable to get window width")
        .as_f64()
        .expect("cant convert to f64");
    let window_height = window
        .inner_height()
        .expect("unable to get height")
        .as_f64()
        .expect("cant convert to f64");
    (window_width as i32, window_height as i32)
}

/// set the title of the document
pub fn set_window_title(title: &str) {
    document().set_title(title);
}
