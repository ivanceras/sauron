use crate::dom::window;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

/// request animation frame handle
#[derive(Clone)]
pub struct AnimationFrameHandle {
    handle: i32,
    _closure: Rc<Closure<dyn FnMut()>>,
}
impl Drop for AnimationFrameHandle {
    fn drop(&mut self) {
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
        _closure: Rc::new(closure),
    })
}
