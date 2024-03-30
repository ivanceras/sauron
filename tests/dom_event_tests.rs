use sauron::{html::attributes::*, html::events::*, html::*, *};
use std::{cell::RefCell, rc::Rc};
use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

// Make sure that we successfully attach an event listener and see it work.
#[wasm_bindgen_test]
fn on_input_test() {
    console_log::init_with_level(log::Level::Trace).ok();
    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "input-element-1";

    let input: Node<()> = input(
        vec![
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            on_input(move |event: InputEvent| {
                *text_clone.borrow_mut() = event.value();
            }),
            value("End Text"),
        ],
        vec![],
    );

    let input_event = web_sys::InputEvent::new("input").unwrap();

    let mut simple_program = simple_program();
    simple_program
        .update_dom_with_vdom(input)
        .expect("must not error");

    let input_element = sauron_core::dom::document()
        .get_element_by_id(elem_id)
        .unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // After dispatching the on_input event our `text` should have a value of the input elements value.
    web_sys::EventTarget::from(input_element)
        .dispatch_event(&input_event)
        .unwrap();

    assert_eq!(&*text.borrow(), "End Text");
}

#[wasm_bindgen_test]
fn added_event() {
    console_log::init_with_level(log::Level::Trace).ok();
    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "input-add-event-test";

    let old: Node<()> = input(
        vec![
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            value("End Text"),
        ],
        vec![],
    );

    let new = input(
        vec![
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            value("End Text"),
            on_input(move |event: InputEvent| {
                log::info!("input event is triggered..");
                *text_clone.borrow_mut() = event.value();
            }),
        ],
        vec![],
    );

    let input_event = web_sys::InputEvent::new("input").unwrap();

    let mut simple_program = simple_program();

    simple_program
        .update_dom_with_vdom(old)
        .expect("must update dom");
    // update to new dom with no event attached
    simple_program
        .update_dom_with_vdom(new)
        .expect("must not error");

    let document = sauron_core::dom::document();
    let input_element = document
        .get_element_by_id(elem_id)
        .unwrap();
    log::info!("input_element: {}", input_element.outer_html());
    log::info!("input element: {:#?}", input_element);

    assert_eq!(&*text.borrow(), "Start Text");

    // Dispatching the event, after the dom is updated
    let ret = web_sys::EventTarget::from(input_element)
        .dispatch_event(&input_event);
    log::info!("dispatched ret: {:?}", ret);

    // TODO: this seems to be not working anymore
    //Should change the text
    assert_eq!(&*text.borrow(), "End Text");
}

#[wasm_bindgen_test]
fn remove_event() {
    console_log::init_with_level(log::Level::Trace).ok();
    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "input-remove-event-test";

    let old: Node<()> = input(
        vec![
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            value("End Text"),
            on_input(move |event: InputEvent| {
                *text_clone.borrow_mut() = event.value();
            }),
        ],
        vec![],
    );

    let new = input(
        vec![
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            value("End Text"),
        ],
        vec![],
    );
    let patch = diff(&old, &new);
    log::debug!("patch: {:?}", patch);

    let input_event = web_sys::InputEvent::new("input").unwrap();

    let mut simple_program = simple_program();
    simple_program
        .update_dom_with_vdom(old)
        .expect("must update dom");
    // update to new dom with no event attached
    simple_program
        .update_dom_with_vdom(new)
        .expect("must not error");

    let input_element = sauron_core::dom::document()
        .get_element_by_id(elem_id)
        .unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // Dispatching the event, after the dom is updated
    web_sys::EventTarget::from(input_element)
        .dispatch_event(&input_event)
        .unwrap();

    //Should never change the text, since it is removed with the dom_updater.update is called with
    //the `new` vdom which has no attached event
    assert_eq!(&*text.borrow(), "Start Text");
}
