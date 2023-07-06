//! utility functions
//!
use js_sys::Promise;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
pub use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;

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

/// utility function which a closure in request animation frame
pub fn request_animation_frame<F>(f: F) -> Result<i32, JsValue>
where
    F: FnMut() + 'static,
{
    let closure_raf: Closure<dyn FnMut() + 'static> = Closure::once(f);
    let handle = window().request_animation_frame(closure_raf.as_ref().unchecked_ref())?;
    closure_raf.forget();
    Ok(handle)
}

/// cancel the animation frame with handle
pub fn cancel_animation_frame(handle: i32) -> Result<(), JsValue>{
    window().cancel_animation_frame(handle)
}


/// request and idle callback
pub fn request_idle_callback<F>(f: F) -> Result<u32, JsValue>
where
    F: Fn(web_sys::IdleDeadline) + 'static,
{
    let closure = Closure::once(move |v: JsValue| {
        let deadline = v
            .dyn_into::<web_sys::IdleDeadline>()
            .expect("must have an idle deadline");
        f(deadline);
    });

    let handle = window().request_idle_callback(closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(handle)
}

/// cancel the idle callback with handle
pub fn cancel_idle_callback(handle: u32) -> Result<(), JsValue>{
    window().cancel_idle_callback(handle);
    Ok(())
}

/// request and idle callback
pub fn request_timeout_callback<F>(f: F, timeout: i32) -> Result<i32, JsValue>
where
    F: FnMut() + 'static,
{
    let closure = Closure::once(f);
    let handle = window().set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), timeout)?;
    closure.forget();
    Ok(handle)
}

/// cancel the timeout callback with handle
pub fn cancel_timeout_callback(handle: i32) -> Result<(), JsValue>{
    window().clear_timeout_with_handle(handle);
    Ok(())
}

/// execute the function at a certain specified timeout in ms
pub fn delay_exec<F>(f: F, timeout: i32) -> Result<i32, JsValue>
where
    F: FnMut() + 'static,
{
    let closure_delay = Closure::once(f);
    let timeout_id = window().set_timeout_with_callback_and_timeout_and_arguments_0(
        closure_delay.as_ref().unchecked_ref(),
        timeout,
    );
    closure_delay.forget();
    timeout_id
}

/// inject style to document head
pub fn inject_style(style: &str) {
    let head = document().head().expect("must have a head");
    let style_node = document().create_element("style").expect("create style element");
    let style_css = document().create_text_node(style);
    style_node.append_child(&style_css).expect("append to style");
    head.append_child(&style_node).expect("must append to head");
}


/// simulate a delay using promise in js
pub async fn async_delay(timeout: i32) {
    let promise = Promise::new(&mut |resolve, _reject| {
        let _handle = delay_exec(
            move || {
                resolve
                    .call0(&JsValue::NULL)
                    .expect("must be able to call resolve");
            },
            timeout,
        ).expect("must schedule it");
    });
    JsFuture::from(promise).await.expect("must not error");
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
