use crate::dom::task::RecurringTask;
use crate::{
    dom::{dom_node::intern, util, window, Application, Program, Task},
};
use futures::channel::mpsc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::MouseEvent;

/// Provides function for window related functions
#[derive(Clone, Copy)]
pub struct Window;

impl Window {
    /// Create a recurring Task which will be triggered
    /// everytime the window is resized
    pub fn on_resize<F, MSG>(mut cb: F) -> Task<MSG>
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

        Task::Recurring(RecurringTask { receiver: rx,
            event_closures: vec![resize_callback],
        })
    }

    pub fn on_mousemove<F, MSG>(mut cb: F) -> Task<MSG>
        where F: FnMut(web_sys::MouseEvent) -> MSG + Clone + 'static,
              MSG: 'static,
    {
        let (mut tx, rx) = mpsc::unbounded();
        let mousemove_cb: Closure<dyn FnMut(web_sys::Event)> = Closure::new(move|event: web_sys::Event|{
            let mouse_event: MouseEvent = event.dyn_into().expect("must be mouse event");
            let msg = cb(mouse_event);
            tx.start_send(msg).expect("send");
        });
        window().add_event_listener_with_callback(intern("mousemove"), mousemove_cb.as_ref().unchecked_ref())
            .expect("add event callback");
        Task::Recurring(RecurringTask{receiver: rx,
            event_closures: vec![mousemove_cb]
        })
    }

    pub fn on_mouseup<F, MSG>(mut cb: F) -> Task<MSG>
        where F: FnMut(web_sys::MouseEvent) -> MSG + Clone + 'static,
              MSG: 'static,
    {
        let (mut tx, rx) = mpsc::unbounded();
        let mousemove_cb: Closure<dyn FnMut(web_sys::Event)> = Closure::new(move|event: web_sys::Event|{
            let mouse_event: MouseEvent = event.dyn_into().expect("must be mouse event");
            let msg = cb(mouse_event);
            tx.start_send(msg).expect("send");
        });
        window().add_event_listener_with_callback(intern("mouseup"), mousemove_cb.as_ref().unchecked_ref())
            .expect("add event callback");
        Task::Recurring(RecurringTask{receiver: rx,
            event_closures: vec![mousemove_cb]
        })
    }
}

impl<APP> Program<APP>
where
    APP: Application,
{
    /*
    /// attach event listeners to the window object
    pub fn add_window_event_listeners(&self, event_listeners: Vec<Attribute<APP::MSG>>) {
        self.add_event_listeners(&window(), event_listeners)
            .expect("must add to event listener");
    }

    /// attach event listeners to the document object
    pub fn add_document_event_listeners(&self, event_listeners: Vec<Attribute<APP::MSG>>) {
        self.add_event_listeners(&document(), event_listeners)
            .expect("must add to event listener");
    }
    */

    /*
    /// attached a callback and will be triggered when the hash portion of the window location
    /// url is changed
    pub fn on_hashchange<F>(&self, mut cb: F)
    where
        F: FnMut(String) -> APP::MSG + 'static,
    {
        let program = Program::downgrade(self);
        let closure: Closure<dyn FnMut(web_sys::Event)> = Closure::new(move |_| {
            let hash = util::get_location_hash();
            let msg = cb(hash);
            let mut program = program.upgrade().expect("must upgrade");
            program.dispatch(msg);
        });
        window().set_onhashchange(Some(closure.as_ref().unchecked_ref()));
        self.event_closures.borrow_mut().push(closure);
    }
    */
}
