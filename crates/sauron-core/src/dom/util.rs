use wasm_bindgen::{closure::Closure, JsCast};
use js_sys::Promise;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
pub use wasm_bindgen_futures::spawn_local;

thread_local!(static WINDOW: web_sys::Window = web_sys::window().expect("no global `window` exists"));
/// utility function which returns the Window element
pub fn window() -> web_sys::Window {
    WINDOW.with(|window| window.clone())
}

/// utility function which returns the history api of the browser
pub fn history() -> web_sys::History {
    window().history().expect("should have a history object")
}

/// utility function which a closure in request animation frame
pub fn request_animation_frame<F>(f: F)
where
    F: FnMut() + 'static,
{
    let closure_raf: Closure<dyn FnMut() + 'static> = Closure::once(f);
    request_animation_frame_for_closure(closure_raf)
}

/// utility function which executes the agument closure in a request animation frame
pub(crate) fn request_animation_frame_for_closure(f: Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
    f.forget()
}

/// execute the function at a certain specified timeout in ms
pub fn delay_exec(
    closure_delay: Closure<dyn FnMut()>,
    timeout: i32,
) -> Option<i32> {
    let timeout_id = window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure_delay.as_ref().unchecked_ref(),
            timeout,
        )
        .expect("should register the setTimeout call");

    closure_delay.forget();

    Some(timeout_id)
}


fn future_delay(timeout: i32) -> JsFuture{
    let promise = Promise::new(&mut |resolve, _reject|{
        delay_exec(Closure::once(move||{
            resolve.call0(&JsValue::NULL).expect("must be able to call resolve");
        }),
        timeout);
    });
    JsFuture::from(promise)
}

/// simulate a delay using promise in js
pub async fn async_delay(timeout:i32) {
    future_delay(timeout).await.expect("must not error");
}

thread_local!(static DOCUMENT: web_sys::Document = window().document().expect("should have a document on window"));
/// provides access to the document element
pub fn document() -> web_sys::Document {
    DOCUMENT.with(|document| document.clone())
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
