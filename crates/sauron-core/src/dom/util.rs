//! utility functions
//!
pub use wasm_bindgen_futures::spawn_local;

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
