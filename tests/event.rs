#![deny(warnings)]
#![feature(proc_macro_hygiene)]

extern crate wasm_bindgen_test;
extern crate web_sys;
use std::rc::Rc;
use wasm_bindgen_test::*;

use sauron::dom::DomUpdater;
use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::*;
use std::cell::RefCell;
use web_sys::*;

wasm_bindgen_test_configure!(run_in_browser);

// Make sure that we successfully attach an event listener and see it work.
#[wasm_bindgen_test]
fn on_input() {
    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "input-element-1";

    let input = input(
        [
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            oninput(move |event: sauron_vdom::Event| match event {
                sauron_vdom::Event::InputEvent(input) => {
                    *text_clone.borrow_mut() = input.value;
                }
                _ => unimplemented!(),
            }),
            value("End Text"),
        ],
        [],
    );

    let input_event = InputEvent::new("input").unwrap();

    let body = sauron::body();
    let _dom_updater = DomUpdater::new_append_to_mount(input, &body);

    let input_element = document().get_element_by_id(&elem_id).unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // After dispatching the oninput event our `text` should have a value of the input elements value.
    web_sys::EventTarget::from(input_element)
        .dispatch_event(&input_event)
        .unwrap();

    assert_eq!(&*text.borrow(), "End Text");
}
