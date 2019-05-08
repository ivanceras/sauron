use crate::Dispatch;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure,
                   JsCast};

/// provides an interface for doing url request, such as fetch
/// resize events, keyboard event, timeout event
pub struct Browser;

impl Browser {
    pub fn onresize<F, DSP, MSG>(program: &Rc<DSP>, cb: F)
        where F: Fn(i32, i32) -> MSG + 'static,
              DSP: Dispatch<MSG> + 'static
    {
        let program_clone = Rc::clone(program);
        let resize_callback: Closure<Fn(web_sys::Event)> =
            Closure::wrap(Box::new(move |_| {
                              let (window_width, window_height) =
                                  Self::get_size();
                              let msg = cb(window_width, window_height);
                              program_clone.dispatch(msg);
                          }));
        crate::window().set_onresize(Some(resize_callback.as_ref()
                                                         .unchecked_ref()));
        resize_callback.forget();
    }

    fn get_size() -> (i32, i32) {
        let window = crate::window();
        let window_width = window.inner_width()
                                 .expect("unable to get window width")
                                 .as_f64()
                                 .expect("cant convert to f64");
        let window_height = window.inner_height()
                                  .expect("unable to get height")
                                  .as_f64()
                                  .expect("cant convert to f64");
        (window_width as i32, window_height as i32)
    }
}
