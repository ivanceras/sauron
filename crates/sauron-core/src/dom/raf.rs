use crate::dom::window;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

/// request animation frame handle
pub struct AnimationFrameHandle {
    handle: i32,
    _closure: Closure<dyn FnMut()>,
}
impl Drop for AnimationFrameHandle {
    fn drop(&mut self) {
        log::debug!("dropping animation frame handle..");
        window()
            .cancel_animation_frame(self.handle)
            .expect("cancel animation handle")
    }
}

/// utility function which a closure in request animation frame
pub fn request_animation_frame<F>(f: F) -> Result<AnimationFrameHandle, JsValue>
where
    F: FnMut() + 'static,
{
    let closure: Closure<dyn FnMut() + 'static> = Closure::once(f);
    let handle = window().request_animation_frame(closure.as_ref().unchecked_ref())?;
    Ok(AnimationFrameHandle {
        handle,
        _closure: closure,
    })
}
