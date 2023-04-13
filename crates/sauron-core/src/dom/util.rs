#![allow(unused)]
use js_sys::Promise;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
pub use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;

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
pub fn request_animation_frame<F>(f: F) -> Result<i32, JsValue>
where
    F: FnMut() + 'static,
{
    let closure_raf: Closure<dyn FnMut() + 'static> = Closure::once(f);
    request_animation_frame_for_closure(closure_raf)
}

/// utility function which executes the agument closure in a request animation frame
pub(crate) fn request_animation_frame_for_closure(
    f: Closure<dyn FnMut()>,
) -> Result<i32, JsValue> {
    let handle = window()
        .request_animation_frame(f.as_ref().unchecked_ref())?;

    f.forget();
    Ok(handle)
}

pub fn request_idle_callback<F>(f: F) -> Result<u32, JsValue>
where
    F: Fn(JsValue) + 'static,
{
    let closure_raf: Closure<dyn Fn(JsValue) + 'static> =
        Closure::wrap(Box::new(f));
    request_idle_callback_for_closure(closure_raf)
}

pub(crate) fn request_idle_callback_for_closure(
    f: Closure<dyn Fn(JsValue)>,
) -> Result<u32, JsValue> {
    let handle = window()
        .request_idle_callback(f.as_ref().unchecked_ref())?;

    f.forget();
    Ok(handle)
}

pub(crate) fn request_idle_callback_with_deadline<F>(
    f: F,
) -> Result<u32, JsValue>
where
    F: Fn(f64) + 'static,
{
    request_idle_callback(move |v: JsValue| {
        let deadline = v
            .dyn_into::<web_sys::IdleDeadline>()
            .expect("must have an idle deadline")
            .time_remaining();
        f(deadline);
    })
}

/// execute the function at a certain specified timeout in ms
pub fn delay_exec_with_closure(
    closure_delay: Closure<dyn FnMut()>,
    timeout: i32,
) -> Result<i32, JsValue> {
    let timeout_id = window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure_delay.as_ref().unchecked_ref(),
            timeout,
        );

    closure_delay.forget();

    timeout_id
}

/// execute with a timeout delay
pub fn delay_exec<F>(
    mut f: F,
    timeout: i32,
) -> Result<i32, JsValue>
where F: FnMut() + 'static
{
    delay_exec_with_closure(Closure::once(move||{
        f()
    }), timeout)
}

/// cancel the execution of a delayed closure
pub fn clear_timeout_with_handle(handle: i32) {
    window().clear_timeout_with_handle(handle)
}

fn future_delay(timeout: i32) -> (JsFuture, Option<i32>) {
    let mut handle = None;
    let promise = Promise::new(&mut |resolve, _reject| {
        if let Ok(ret) = delay_exec_with_closure(
            Closure::once(move || {
                resolve
                    .call0(&JsValue::NULL)
                    .expect("must be able to call resolve");
            }),
            timeout,
        ){
            handle = Some(ret);
        }
    });
    (JsFuture::from(promise), handle)
}

/// simulate a delay using promise in js
pub async fn async_delay(timeout: i32) {
    let (fut, _handle) = future_delay(timeout);
    fut.await.expect("must not error");
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
