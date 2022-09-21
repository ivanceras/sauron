#![deny(warnings)]
use sauron::{
    mt_dom::TreePath,
    prelude::*,
};
use std::{
    cell::RefCell,
    rc::Rc,
};
use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn elements_with_different_event_should_not_be_recycle() {
    console_log::init_with_level(log::Level::Trace).ok();
    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "input-add-event-test";

    let old = input(
        vec![
            id(elem_id),
            on_input(move |_event: InputEvent| {
                *text_clone.borrow_mut() = "Old value".to_string();
            }),
        ],
        vec![],
    );

    let old_clone = old.clone();
    dbg!(&old);
    dbg!(&old_clone);
    assert_eq!(old, old_clone);

    let text_clone2 = Rc::clone(&text);

    let cb2 = on_input(move |_event: InputEvent| {
        *text_clone2.borrow_mut() = "New value".to_string();
    });
    let new = input(vec![id(elem_id), cb2.clone()], vec![]);

    let patches: Vec<Patch<()>> = diff(&old, &new);
    // FIXME: this should replace the old node with a new one since the even essentially is a new
    // one. But we have no way of knowing the closure will produce the same result of not
    log::trace!("patches: {:#?}", patches);

    //should contain AddAttributes instead of empty.
    assert_eq!(
        patches,
        vec![Patch::add_attributes(
            &"input",
            TreePath::new(vec![]),
            vec![&cb2]
        )]
    );

    let input_event = web_sys::InputEvent::new("input").unwrap();

    let body = sauron_core::body();
    let simple_program = simple_program();
    let mut dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, old.clone(), &body);

    // update to new dom with no event attached
    dom_updater.patch_dom(&simple_program, patches);

    let input_element =
        sauron_core::document().get_element_by_id(elem_id).unwrap();

    // before clicking
    assert_eq!(&*text.borrow(), "Start Text");

    // Dispatching the event, after the dom is updated
    web_sys::EventTarget::from(input_element)
        .dispatch_event(&input_event)
        .unwrap();

    //FIXME: the new event should be triggered instead of the old one
    //Should change the text
    assert_eq!(&*text.borrow(), "New value");
}
