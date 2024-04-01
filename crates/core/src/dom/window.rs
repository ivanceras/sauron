use crate::dom::{dom_node::intern, util, window, Cmd};
use futures::channel::mpsc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::MouseEvent;

/// Provides function for window related functions
#[derive(Clone, Copy)]
pub struct Window;

impl Window {
    /// Create a recurring Cmd which will be triggered
    /// everytime the window is resized
    pub fn on_resize<F, MSG>(mut cb: F) -> Cmd<MSG>
    where
        F: FnMut(i32, i32) -> MSG + Clone + 'static,
        MSG: 'static,
    {
        let (mut tx, rx) = mpsc::unbounded();
        let resize_callback: Closure<dyn FnMut(web_sys::Event)> =
            Closure::new(move |e: web_sys::Event| {
                log::info!("event: {}", e.type_());
                let (w, h) = util::get_window_size();
                let msg = cb(w, h);
                tx.start_send(msg).expect("send");
            });
        window()
            .add_event_listener_with_callback(
                intern("resize"),
                resize_callback.as_ref().unchecked_ref(),
            )
            .expect("add event callback");

        Cmd::sub(rx, resize_callback)
    }

    ///
    pub fn on_mousemove<F, MSG>(mut cb: F) -> Cmd<MSG>
    where
        F: FnMut(web_sys::MouseEvent) -> MSG + Clone + 'static,
        MSG: 'static,
    {
        let (mut tx, rx) = mpsc::unbounded();
        let mousemove_cb: Closure<dyn FnMut(web_sys::Event)> =
            Closure::new(move |event: web_sys::Event| {
                let mouse_event: MouseEvent = event.dyn_into().expect("must be mouse event");
                let msg = cb(mouse_event);
                tx.start_send(msg).expect("send");
            });
        window()
            .add_event_listener_with_callback(
                intern("mousemove"),
                mousemove_cb.as_ref().unchecked_ref(),
            )
            .expect("add event callback");
        Cmd::sub(rx, mousemove_cb)
    }

    ///
    pub fn on_mouseup<F, MSG>(mut cb: F) -> Cmd<MSG>
    where
        F: FnMut(web_sys::MouseEvent) -> MSG + Clone + 'static,
        MSG: 'static,
    {
        let (mut tx, rx) = mpsc::unbounded();
        let mousemove_cb: Closure<dyn FnMut(web_sys::Event)> =
            Closure::new(move |event: web_sys::Event| {
                let mouse_event: MouseEvent = event.dyn_into().expect("must be mouse event");
                let msg = cb(mouse_event);
                tx.start_send(msg).expect("send");
            });
        window()
            .add_event_listener_with_callback(
                intern("mouseup"),
                mousemove_cb.as_ref().unchecked_ref(),
            )
            .expect("add event callback");
        Cmd::sub(rx, mousemove_cb)
    }

    /// do this task at every `ms` interval
    pub fn every_interval<F, MSG>(interval_ms: i32, cb: F) -> Cmd<MSG>
    where
        F: Fn() -> MSG + 'static,
        MSG: 'static,
    {
        let (mut tx, rx) = mpsc::unbounded();
        //The web_sys::Event here is undefined, it is just used here to make storing the closure
        //uniform
        let closure_cb: Closure<dyn FnMut(web_sys::Event)> = Closure::new(move |_event| {
            log::info!("event: {:?}", _event);
            let msg = cb();
            tx.start_send(msg).unwrap();
        });
        window()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                closure_cb.as_ref().unchecked_ref(),
                interval_ms,
            )
            .expect("Unable to start interval");
        Cmd::sub(rx, closure_cb)
    }

    /// scroll the window to the top of the document
    pub fn scroll_to_top<MSG>(msg: MSG) -> Cmd<MSG>
    where
        MSG: 'static,
    {
        use std::future::ready;
        Cmd::single(ready({
            util::scroll_window_to_top();
            msg
        }))
    }

    ///
    pub fn on_popstate<F, MSG>(mut cb: F) -> Cmd<MSG>
    where
        F: FnMut(web_sys::PopStateEvent) -> MSG + 'static,
        MSG: 'static,
    {
        let (mut tx, rx) = mpsc::unbounded();
        let closure_cb: Closure<dyn FnMut(web_sys::Event)> =
            Closure::new(move |event: web_sys::Event| {
                let popstate_event: web_sys::PopStateEvent =
                    event.dyn_into().expect("popstate event");
                let msg = cb(popstate_event);
                tx.start_send(msg).expect("send");
            });
        window()
            .add_event_listener_with_callback(
                intern("mouseup"),
                closure_cb.as_ref().unchecked_ref(),
            )
            .expect("add event callback");
        Cmd::sub(rx, closure_cb)
    }
}
