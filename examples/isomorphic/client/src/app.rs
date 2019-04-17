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

use sauron::Node;
pub use store::{Msg, Store};

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

impl Component<Msg> for App {
    fn update(&mut self, msg: &Msg) {
        match msg {
            Msg::Click => crate::log("increment something"),
            Msg::Clock => crate::log("tick tok"),
        }
    }

    fn view(&self) -> Node<Msg> {
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
                            Msg::Click
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
                                    Msg::Click
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
                                    Msg::Click
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
                                    Msg::Click
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
