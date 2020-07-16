use crate::{
    Cmd,
    Component,
    Dispatch,
};
use std::fmt::Debug;
use wasm_bindgen::{
    closure::Closure,
    JsCast,
};
use web_sys::ScrollToOptions;

/// provides an interface for doing url request, such as fetch
/// resize events, keyboard event, timeout event
#[derive(Copy, Clone, Debug)]
pub struct Browser;

impl Browser {
    /// Creates a Cmd in which the MSG will be emitted
    /// whenever the browser is resized
    pub fn on_resize<F, APP, MSG>(cb: F) -> Cmd<APP, MSG>
    where
        F: Fn(i32, i32) -> MSG + Clone + 'static,
        MSG: PartialEq + Debug + Clone + 'static,
        APP: Component<MSG> + 'static,
    {
        let cmd: Cmd<APP, MSG> = Cmd::new(move |program| {
            let cb_clone = cb.clone();
            let resize_callback: Closure<dyn Fn(web_sys::Event)> =
                Closure::wrap(Box::new(move |_| {
                    let (window_width, window_height) = Self::get_size();
                    let msg = cb_clone(window_width, window_height);
                    program.dispatch(msg);
                }));
            crate::window()
                .set_onresize(Some(resize_callback.as_ref().unchecked_ref()));
            resize_callback.forget();
        });
        cmd
    }

    /// attached a callback and will be triggered when the hash portion of the window location
    /// url is changed
    pub fn on_hashchange<F, APP, MSG>(cb: F) -> Cmd<APP, MSG>
    where
        F: Fn(String) -> MSG + Clone + 'static,
        MSG: 'static,
        APP: Component<MSG> + 'static,
    {
        let cmd: Cmd<APP, MSG> = Cmd::new(move |program| {
            let cb_clone = cb.clone();
            let hashchange_callback: Closure<dyn Fn(web_sys::Event)> =
                Closure::wrap(Box::new(move |_| {
                    let hash = Self::get_hash();
                    let msg = cb_clone(hash);
                    program.dispatch(msg);
                }));
            crate::window().set_onhashchange(Some(
                hashchange_callback.as_ref().unchecked_ref(),
            ));
            hashchange_callback.forget();
        });
        cmd
    }

    fn get_size() -> (i32, i32) {
        let window = crate::window();
        let window_width = window
            .inner_width()
            .expect("unable to get window width")
            .as_f64()
            .expect("cant convert to f64");
        let window_height = window
            .inner_height()
            .expect("unable to get height")
            .as_f64()
            .expect("cant convert to f64");
        (window_width as i32, window_height as i32)
    }

    /// return the hash part of the browser current url location
    /// The hash part are the text right after the `#` sign
    pub fn get_hash() -> String {
        let window = crate::window();
        let hash = window.location().hash().expect("must have a hash");
        hash
    }

    /// scroll the browser to the top of the document
    pub fn scroll_to_top() {
        let mut options = ScrollToOptions::new();
        options.top(0.0);
        options.left(0.0);
        crate::window().scroll_to_with_scroll_to_options(&options);
    }
}
