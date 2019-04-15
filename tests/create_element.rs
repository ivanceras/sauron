#![deny(warnings)]
use sauron::dom::CreatedNode;
use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::svg::attributes::*;
use sauron::svg::*;
use sauron::*;
use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use web_sys::{console, Element, Event, EventTarget};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn nested_divs() {
    let vdiv = div([], [div([], [div([], [])])]); // <div> <div> <div></div> </div> </div>
    let div: Element = CreatedNode::<Element>::create_dom_node(&vdiv)
        .node
        .unchecked_into();

    assert_eq!(&div.inner_html(), "<div><div></div></div>");
}

#[wasm_bindgen_test]
fn svg_element() {
    let vdiv = div(
        [],
        [svg(
            [xmlns("http://www.w3.org/2000/svg")],
            [circle([cx("50"), cy("50"), r("50")], [])],
        )],
    );
    let div: Element = CreatedNode::<Element>::create_dom_node(&vdiv)
        .node
        .unchecked_into();

    assert_eq!(
        &div.inner_html(),
        r#"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="50" cy="50" r="50"></circle></svg>"#
    );
}

#[wasm_bindgen_test]
fn div_with_attributes() {
    let vdiv = div([id("id-here"), class("two classes")], []);
    let div: Element = CreatedNode::<Element>::create_dom_node(&vdiv)
        .node
        .unchecked_into();

    assert_eq!(&div.id(), "id-here");

    assert!(div.class_list().contains("two"));;
    assert!(div.class_list().contains("classes"));;

    assert_eq!(div.class_list().length(), 2);
}

//FIXME: this fails because the closure is already dropped
// when the event is dispatched
// TODO: This now passed, when `closure_wrap.forget()` is called
// in `DomUpdater.create_element_node`, but then the closures
// will be leaking and there is no way to remove that closure from
// the event listener.
#[wasm_bindgen_test]
fn click_event() {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    let clicked = Rc::new(Cell::new(false));
    let clicked_clone = Rc::clone(&clicked);

    let elem_id = "click-on-div";
    let vdiv = div(
        [
            id(elem_id),
            onclick(move |_ev: sauron_vdom::Event| {
                console::log_1(&"clicked event called".into());
                clicked_clone.set(true);
            }),
        ],
        [],
    );

    let _dom_updater = DomUpdater::new_append_to_mount(vdiv, &body);

    let click_event = Event::new("click").unwrap();

    let div = document.get_element_by_id(&elem_id).unwrap();

    (EventTarget::from(div))
        .dispatch_event(&click_event)
        .unwrap();

    assert_eq!(*clicked, Cell::new(true));
}
