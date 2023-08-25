#![deny(warnings)]
use sauron::prelude::*;
use std::{cell::RefCell, rc::Rc};
use test_fixtures::simple_program;
use wasm_bindgen_test::*;
use web_sys::InputEvent;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

// Verify that our DomUpdater's patch method works.
// We test a simple case here, since diff_patch.rs is responsible for testing more complex
// diffing and patching.
#[wasm_bindgen_test]
fn patches_dom() {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let vdom: Node<()> = div(vec![], vec![]);
    let mut simple_program = simple_program();
    simple_program
        .update_dom_with_vdom(vdom)
        .expect("must not error");

    let new_vdom = div(vec![id("patched")], vec![]); //html! { <div id="patched"></div> };
    simple_program
        .update_dom_with_vdom(new_vdom)
        .expect("must not error");

    assert_eq!(document.query_selector("#patched").unwrap().is_some(), true);
}

// When you replace a DOM node with another DOM node we need to make sure that the closures
// from the new DOM node are stored by the DomUpdater otherwise they'll get dropped and
// won't work.
#[wasm_bindgen_test]
fn updates_active_closure_on_replace() {
    console_error_panic_hook::set_once();

    let mut simple_program = simple_program();
    let old = div(vec![], vec![]);

    simple_program
        .update_dom_with_vdom(old)
        .expect("must update dom");

    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "update-active-closures-on-replace";

    let replace_node = input(
        vec![
            id(elem_id),
            on_input(move |event: sauron_core::html::events::InputEvent| {
                *text_clone.borrow_mut() = event.value();
            }),
            value("End Text"),
        ],
        vec![],
    );

    // New node replaces old node.
    // We are testing that we've stored this new node's closures even though `new` will be dropped
    // at the end of this block.
    simple_program
        .update_dom_with_vdom(replace_node)
        .expect("must not error");

    let input_event = InputEvent::new("input").unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // After dispatching the on_input event our `text` should have a value of the input elements value.
    let input = sauron_core::dom::document()
        .get_element_by_id(&elem_id)
        .unwrap();
    web_sys::EventTarget::from(input)
        .dispatch_event(&input_event)
        .unwrap();

    assert_eq!(&*text.borrow(), "End Text");
}

// When you replace a DOM node with another DOM node we need to make sure that the closures
// from the new DOM node are stored by the DomUpdater otherwise they'll get dropped and
// won't work.
#[wasm_bindgen_test]
async fn updates_active_closures_on_append() {
    console_error_panic_hook::set_once();

    let old = div(vec![], vec![]);
    let mut simple_program = simple_program();
    simple_program
        .update_dom_with_vdom(old)
        .expect("must update dom");

    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "update-active-closures-on-append";

    {
        let append_node = div(
            vec![],
            vec![input(
                vec![
                    id(elem_id),
                    on_input(move |event: sauron_core::html::events::InputEvent| {
                        *text_clone.borrow_mut() = event.value();
                    }),
                    value("End Text"),
                ],
                vec![],
            )],
        );

        // New node gets appended into the DOM.
        // We are testing that we've stored this new node's closures even though `new` will be dropped
        // at the end of this block.
        simple_program
            .update_dom_with_vdom(append_node)
            .expect("must not error");
    }

    let input_event = InputEvent::new("input").unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // After dispatching the on_input event our `text` should have a value of the input elements value.
    let input = sauron_core::dom::document()
        .get_element_by_id(elem_id)
        .unwrap();
    web_sys::EventTarget::from(input)
        .dispatch_event(&input_event)
        .unwrap();

    assert_eq!(&*text.borrow(), "End Text");
}
