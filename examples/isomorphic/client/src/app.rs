use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::Event;
use sauron::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use sauron::Component;
use sauron::Node;
use sauron::View;
use sauron::Widget;
use store::{Msg, Store};

mod store;

pub struct App {
    pub store: Rc<RefCell<Store>>,
}

impl App {
    pub fn new(count: u32) -> App {
        let store = Store::new(count);

        let rc_store = Rc::new(RefCell::new(store));
        let store_clone = Rc::clone(&rc_store);

        let clock = Closure::wrap(
            Box::new(move || store_clone.borrow_mut().msg(&Msg::Clock)) as Box<dyn Fn()>
        );
        window()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                clock.as_ref().unchecked_ref(),
                1000,
            )
            .expect("Unable to start interval");
        clock.forget();

        App { store: rc_store }
    }
}

impl Component for App {
    /// Whatever changes in the store the callback
    /// will be called
    fn subscribe(&mut self, callback: Box<Fn()>) {
        self.store.borrow_mut().subscribe(callback);
    }
}

impl Widget for App {
    fn update(&mut self) {
        // nothing to update
    }
}

impl View for App {
    fn view(&self) -> Node {
        let store_clone = Rc::clone(&self.store);
        let count: u32 = self.store.borrow().click_count();
        let current_time = self
            .store
            .borrow()
            .time()
            .to_locale_string("en-GB", &JsValue::undefined());
        div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [
                div([], [text(format!("Hello world! {}", count))]),
                div([id("current-time")], [text(current_time)]),
                div(
                    [],
                    [button(
                        [onclick(move |v: Event| {
                            sauron::log(format!("I've been clicked and the value is: {:#?}", v));
                            store_clone.borrow_mut().msg(&Msg::Click);
                        })],
                        [text("Click me!")],
                    )],
                ),
                div(
                    [],
                    [
                        text("Using oninput"),
                        input(
                            [
                                r#type("text"),
                                oninput(|v: Event| {
                                    sauron::log(format!("input has input: {:#?}", v));
                                }),
                                placeholder("Type here..."),
                            ],
                            [],
                        ),
                    ],
                ),
                div(
                    [],
                    [
                        text("using oninput on a textarea"),
                        textarea(
                            [
                                oninput(|v: Event| {
                                    sauron::log(format!("textarea has changed: {:#?}", v));
                                }),
                                placeholder("Description here..."),
                            ],
                            [],
                        ),
                    ],
                ),
                div(
                    [],
                    [
                        text("Using onchange"),
                        input(
                            [
                                r#type("text"),
                                onchange(|v: Event| {
                                    sauron::log(format!("input has changed: {:#?}", v));
                                }),
                                placeholder("Description here..."),
                            ],
                            [],
                        ),
                    ],
                ),
            ],
        )
    }
}
