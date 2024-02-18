use crate::dom::window;
use js_sys::Promise;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use std::rc::Rc;

/// handle for request_idle_callback calls
#[derive(Debug,Clone)]
pub struct TimeoutCallbackHandle {
    handle: i32,
    _closure: Rc<Closure<dyn FnMut()>>,
}

impl Drop for TimeoutCallbackHandle {
    fn drop(&mut self) {
        window().clear_timeout_with_handle(self.handle);
    }
}

/// request and idle callback
pub fn request_timeout_callback<F>(f: F, timeout: i32) -> Result<TimeoutCallbackHandle, JsValue>
where
    F: FnMut() + 'static,
{
    let closure = Closure::once(f);
    let handle = window().set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        timeout,
    )?;
    Ok(TimeoutCallbackHandle {
        handle,
        _closure: Rc::new(closure),
    })
}

/// simulate a delay using promise in js
pub(crate) async fn async_delay(timeout: i32) -> Result<TimeoutCallbackHandle, JsValue> {
    let mut result = Err(JsValue::NULL);
    let promise = Promise::new(&mut |resolve, _reject| {
        let handle = request_timeout_callback(
            move || {
                resolve
                    .call0(&JsValue::NULL)
                    .expect("must be able to call resolve");
            },
            timeout,
        );
        result = handle;
    });
    JsFuture::from(promise).await.expect("must not error");
    result
}

/// wrapper of async delay but return no result, assume success
pub async fn delay(timeout: i32) {
    async_delay(timeout).await.expect("must not error");
}
