#![deny(warnings)]

use sauron_core::{
    diff,
    dom::DomUpdater,
    html::{attributes::*, events::*, *},
    mt_dom::patch::*,
    web_sys, Node,
};
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
                *text_clone.borrow_mut() = event.value.to_string();
            }),
            value("End Text"),
        ],
        vec![],
    );

    let input_event = web_sys::InputEvent::new("input").unwrap();

    let body = sauron_core::body();
    let simple_program = simple_program();
    let _dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, input, &body);

    let input_element =
        sauron_core::document().get_element_by_id(&elem_id).unwrap();

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
                *text_clone.borrow_mut() = event.value.to_string();
            }),
        ],
        vec![],
    );

    let input_event = web_sys::InputEvent::new("input").unwrap();

    let body = sauron_core::body();
    let simple_program = simple_program();
    let mut dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, old, &body);
    // update to new dom with no event attached
    dom_updater.update_dom(&simple_program, new);

    let input_element =
        sauron_core::document().get_element_by_id(&elem_id).unwrap();

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
                *text_clone.borrow_mut() = event.value.to_string();
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

    let body = sauron_core::body();
    let simple_program = simple_program();
    let mut dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, old, &body);
    // update to new dom with no event attached
    dom_updater.update_dom(&simple_program, new);

    let input_element =
        sauron_core::document().get_element_by_id(&elem_id).unwrap();

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
    console_log::init_with_level(log::Level::Trace).ok();
    let cb = on_click(|_| log::trace!("Clicked here"));
    let old: Node<()> = div(
        vec![],
        vec![
            button(vec![cb.clone()], vec![]),
            button(vec![cb.clone()], vec![]),
            button(vec![cb.clone()], vec![]),
            button(vec![cb.clone()], vec![]),
            button(vec![cb.clone()], vec![]),
        ],
    );

    let new: Node<()> = div(vec![], vec![button(vec![cb.clone()], vec![])]);

    let body = sauron_core::body();
    let simple_program = simple_program();
    let diff = sauron_core::diff(&old, &new);
    log::debug!("{:#?}", diff);
    assert_eq!(
        diff,
        vec![
            RemoveNode::new(Some(&"button"), TreePath::new(vec![0, 1]),).into(),
            RemoveNode::new(Some(&"button"), TreePath::new(vec![0, 2]),).into(),
            RemoveNode::new(Some(&"button"), TreePath::new(vec![0, 3]),).into(),
            RemoveNode::new(Some(&"button"), TreePath::new(vec![0, 4]),).into(),
        ],
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
    console_log::init_with_level(log::Level::Trace).ok();
    let cb = on_click(|_| log::trace!("Clicked here"));
    let old: Node<()> = div(
        vec![],
        vec![
            button(vec![cb.clone()], vec![]),
            button(vec![cb.clone()], vec![]),
            button(vec![], vec![]),
            button(vec![], vec![]),
            button(vec![cb.clone()], vec![]),
        ],
    );

    let new: Node<()> = div(vec![], vec![button(vec![cb.clone()], vec![])]);

    let body = sauron_core::body();
    let simple_program = simple_program();
    let diff = sauron_core::diff(&old, &new);
    log::debug!("{:#?}", diff);
    assert_eq!(
        diff,
        vec![
            RemoveNode::new(Some(&"button"), TreePath::new(vec![0, 1]),).into(),
            RemoveNode::new(Some(&"button"), TreePath::new(vec![0, 2]),).into(),
            RemoveNode::new(Some(&"button"), TreePath::new(vec![0, 3]),).into(),
            RemoveNode::new(Some(&"button"), TreePath::new(vec![0, 4]),).into(),
        ],
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
    console_log::init_with_level(log::Level::Trace).ok();

    let old: Node<()> =
        div(vec![on_click(|_| log::trace!("I'm a div"))], vec![]);

    let new: Node<()> = p(vec![], vec![]);

    let body = sauron_core::body();
    let simple_program = simple_program();
    let diff = sauron_core::diff(&old, &new);
    log::info!("{:#?}", diff);
    assert_eq!(
        diff,
        vec![ReplaceNode::new(
            Some(&"div"),
            TreePath::new(vec![0]),
            &p(vec![], vec![])
        )
        .into()],
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
