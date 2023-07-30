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

#[cfg(feature = "ric-polyfill")]
thread_local!(static RIC_POLYFILL_FUNCTION: js_sys::Function = create_ric_polyfill_function());

#[allow(unused)]
#[cfg(feature = "ric-polyfill")]
pub fn create_ric_polyfill_function() -> js_sys::Function {
    js_sys::Function::new_with_args(
        "cb",
        r#"
        window.requestIdleCallback =
            window.requestIdleCallback ||
            function(cb) {
                console.log("executing requestIdleCallback from the polyfill");
                var start = Date.now();
                return setTimeout(function() {
                    cb({
                        didTimeout: false,
                        timeRemaining: function() {
                            return Math.max(0, 50 - (Date.now() - start));
                        },
                    });
                }, 1);
            };

        window.cancelIdleCallback =
            window.cancelIdleCallback ||
            function(id) {
                clearTimeout(id);
            };
        "#,
    )
}
