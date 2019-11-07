#![deny(warnings)]
#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate log;
extern crate wasm_bindgen_test;
extern crate web_sys;
use std::rc::Rc;
use wasm_bindgen_test::*;

use sauron::{
    dom::DomUpdater,
    html::{
        attributes::*,
        events::*,
        *,
    },
    test_fixtures::simple_program,
    Node,
};
use std::cell::RefCell;

wasm_bindgen_test_configure!(run_in_browser);

// Make sure that we successfully attach an event listener and see it work.
#[wasm_bindgen_test]
fn on_input() {
    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "input-element-1";

    let input: Node<()> = input(
        vec![
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            oninput(move |event: sauron_vdom::event::InputEvent| {
                *text_clone.borrow_mut() = event.value;
            }),
            value("End Text"),
        ],
        vec![],
    );

    let input_event = web_sys::InputEvent::new("input").unwrap();

    let body = sauron::body();
    let simple_program = simple_program();
    let _dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, input, &body);

    let input_element = sauron::document().get_element_by_id(&elem_id).unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // After dispatching the oninput event our `text` should have a value of the input elements value.
    web_sys::EventTarget::from(input_element)
        .dispatch_event(&input_event)
        .unwrap();

    assert_eq!(&*text.borrow(), "End Text");
}

#[wasm_bindgen_test]
fn added_event() {
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
            oninput(move |event: sauron_vdom::event::InputEvent| {
                *text_clone.borrow_mut() = event.value;
            }),
        ],
        vec![],
    );

    let input_event = web_sys::InputEvent::new("input").unwrap();

    let body = sauron::body();
    let simple_program = simple_program();
    let mut dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, old, &body);
    // update to new dom with no event attached
    dom_updater.update_dom(&simple_program, new);

    let input_element = sauron::document().get_element_by_id(&elem_id).unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // Dispatching the event, after the dom is updated
    web_sys::EventTarget::from(input_element)
        .dispatch_event(&input_event)
        .unwrap();

    //Should change the text
    assert_eq!(&*text.borrow(), "End Text");
}

#[wasm_bindgen_test]
fn remove_event() {
    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "input-remove-event-test";

    let old: Node<()> = input(
        vec![
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            value("End Text"),
            oninput(move |event: sauron_vdom::event::InputEvent| {
                *text_clone.borrow_mut() = event.value;
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

    let input_event = web_sys::InputEvent::new("input").unwrap();

    let body = sauron::body();
    let simple_program = simple_program();
    let mut dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, old, &body);
    // update to new dom with no event attached
    dom_updater.update_dom(&simple_program, new);

    let input_element = sauron::document().get_element_by_id(&elem_id).unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // Dispatching the event, after the dom is updated
    web_sys::EventTarget::from(input_element)
        .dispatch_event(&input_event)
        .unwrap();

    //Should never change the text, since it is removed with the dom_updater.update is called with
    //the `new` vdom which has no attached event
    assert_eq!(&*text.borrow(), "Start Text");
}

#[wasm_bindgen_test]
fn remove_event_from_truncated_children() {
    let old: Node<()> = div(
        vec![],
        vec![
            button(vec![onclick(|_| trace!("Clicked here"))], vec![]),
            button(vec![onclick(|_| trace!("Clicked here"))], vec![]),
            button(vec![onclick(|_| trace!("Clicked here"))], vec![]),
            button(vec![onclick(|_| trace!("Clicked here"))], vec![]),
            button(vec![onclick(|_| trace!("Clicked here"))], vec![]),
        ],
    );

    let new: Node<()> = div(
        vec![],
        vec![button(vec![onclick(|_| trace!("Clicked here"))], vec![])],
    );

    let body = sauron::body();
    let simple_program = simple_program();
    assert_eq!(
        sauron::diff(&old, &new),
        vec![sauron_vdom::Patch::TruncateChildren(0, 1)],
        "Should be a Truncate patch"
    );
    let mut dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, old, &body);
    assert_eq!(
        dom_updater.active_closure_len(),
        5,
        "There should be 5 events attached to the DomUpdater"
    );
    dom_updater.update_dom(&simple_program, new);

    assert_eq!(
        dom_updater.active_closure_len(),
        1,
        "There should only be 1 left after the truncate"
    );
}

#[wasm_bindgen_test]
fn remove_event_from_truncated_children_some_with_no_events() {
    let old: Node<()> = div(
        vec![],
        vec![
            button(vec![onclick(|_| trace!("Clicked here"))], vec![]),
            button(vec![onclick(|_| trace!("Clicked here"))], vec![]),
            button(vec![], vec![]),
            button(vec![], vec![]),
            button(vec![onclick(|_| trace!("Clicked here"))], vec![]),
        ],
    );

    let new: Node<()> = div(
        vec![],
        vec![button(vec![onclick(|_| trace!("Clicked here"))], vec![])],
    );

    let body = sauron::body();
    let simple_program = simple_program();
    assert_eq!(
        sauron::diff(&old, &new),
        vec![sauron_vdom::Patch::TruncateChildren(0, 1)],
        "Should be a Truncate patch"
    );
    let mut dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, old, &body);
    assert_eq!(
        dom_updater.active_closure_len(),
        3,
        "There should be 3 events attached to the DomUpdater"
    );
    dom_updater.update_dom(&simple_program, new);

    assert_eq!(
        dom_updater.active_closure_len(),
        1,
        "There should only be 1 left after the truncate"
    );
}

#[wasm_bindgen_test]
fn remove_event_from_replaced_node() {
    let old: Node<()> = div(vec![onclick(|_| trace!("I'm a div"))], vec![]);

    let new: Node<()> = p(vec![], vec![]);

    let body = sauron::body();
    let simple_program = simple_program();
    assert_eq!(
        sauron::diff(&old, &new),
        vec![sauron_vdom::Patch::Replace(
            0,
            &sauron_vdom::Node::Element(sauron_vdom::Element {
                tag: "p",
                attrs: vec![],
                children: vec![],
                namespace: None
            })
        )],
        "Should be a Replace patch"
    );
    let mut dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, old, &body);
    assert_eq!(
        dom_updater.active_closure_len(),
        1,
        "There should be 1 event attached to the DomUpdater"
    );
    dom_updater.update_dom(&simple_program, new);

    assert_eq!(
        dom_updater.active_closure_len(),
        0,
        "There should only be 0 left after replacing it with a different tag"
    );
}
