use crate::dom::window;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

/// request idle callback handle
pub struct IdleCallbackHandle {
    handle: u32,
    _closure: Closure<dyn FnMut(JsValue)>,
}

/// when dropped, cancel the idle callback
impl Drop for IdleCallbackHandle {
    fn drop(&mut self) {
        log::info!("dropping idle callback here..");
        window().cancel_idle_callback(self.handle);
    }
}

/// request and idle callback
pub fn request_idle_callback<F>(mut f: F) -> Result<IdleCallbackHandle, JsValue>
where
    F: FnMut(web_sys::IdleDeadline) + 'static,
{
    let closure = Closure::once(move |v: JsValue| {
        let deadline = v
            .dyn_into::<web_sys::IdleDeadline>()
            .expect("must have an idle deadline");
        f(deadline);
    });

    let handle = window().request_idle_callback(closure.as_ref().unchecked_ref())?;
    Ok(IdleCallbackHandle {
        handle,
        _closure: closure,
    })
}
