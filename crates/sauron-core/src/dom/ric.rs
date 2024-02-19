use crate::dom::{now, request_timeout_callback, window, TimeoutCallbackHandle};
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

/// request idle callback handle
#[derive(Clone)]
pub struct IdleCallbackHandleReal {
    handle: u32,
    _closure: Rc<Closure<dyn FnMut(JsValue)>>,
}

/// when dropped, cancel the idle callback
impl Drop for IdleCallbackHandleReal {
    fn drop(&mut self) {
        window().cancel_idle_callback(self.handle);
    }
}

/// request and idle callback
fn request_idle_callback_real<F>(mut f: F) -> Result<IdleCallbackHandleReal, JsValue>
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
    Ok(IdleCallbackHandleReal {
        handle,
        _closure: Rc::new(closure),
    })
}

/// Idle deadline interface which could be the real idle deadline if supported, otherwise the
/// polyfill
#[derive(Clone)]
pub enum IdleDeadline {
    /// the web native IdleDeadline object wrap together with polyfill version
    Real(web_sys::IdleDeadline),
    /// A polyfill for simulating IdleDeadline
    Polyfill {
        /// timestamp the closure is executed
        start: f64,
    },
}

///
#[derive(Clone)]
pub enum IdleCallbackHandle {
    /// wrapper to the real web native IdleCallbackHandle
    Real(IdleCallbackHandleReal),
    ///
    Polyfill(TimeoutCallbackHandle),
}

impl IdleDeadline {
    fn polyfill() -> Self {
        Self::Polyfill { start: now() }
    }

    /// calculate the remaining time for the IdleDeadline
    pub fn time_remaining(&self) -> f64 {
        match self {
            Self::Real(deadline) => deadline.time_remaining(),
            Self::Polyfill { start } => 0.0_f64.max(50. - now() - start),
        }
    }

    /// returns true if there is no more time for executing more work
    pub fn did_timeout(&self) -> bool {
        match self {
            Self::Real(deadline) => deadline.did_timeout(),
            Self::Polyfill { .. } => self.time_remaining() > 0.,
        }
    }
}

/// request idle callback
pub fn request_idle_callback<F>(mut f: F) -> Result<IdleCallbackHandle, JsValue>
where
    F: FnMut(IdleDeadline) + 'static,
{
    let is_ric_available = window().get("requestIdleCallback").is_some();
    if is_ric_available {
        let handle = request_idle_callback_real(move |dl| {
            let deadline = IdleDeadline::Real(dl);
            f(deadline)
        })?;
        Ok(IdleCallbackHandle::Real(handle))
    } else {
        let handle = request_idle_callback_shim(move || f(IdleDeadline::polyfill()))?;
        Ok(IdleCallbackHandle::Polyfill(handle))
    }
}

fn request_idle_callback_shim<F>(f: F) -> Result<TimeoutCallbackHandle, JsValue>
where
    F: FnMut() + 'static,
{
    request_timeout_callback(f, 1)
}
