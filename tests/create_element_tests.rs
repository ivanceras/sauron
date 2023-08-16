#![deny(warnings)]
use sauron::{
    html::attributes::*,
    html::events::*,
    html::*,
    svg::attributes::{cx, cy, r, xmlns},
    svg::*,
    *,
};
use std::{cell::Cell, rc::Rc};
use test_fixtures::simple_program;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{console, Element, EventTarget};

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn nested_divs() {
    let vdiv: Node<()> = div(vec![], vec![div(vec![], vec![div(vec![], vec![])])]); // <div> <div> <div></div> </div> </div>
    let program = simple_program();
    let created_node = program.create_dom_node(&vdiv);
    let div: Element = created_node.unchecked_into();

    assert_eq!(&div.inner_html(), "<div><div></div></div>");
}

#[wasm_bindgen_test]
fn svg_element() {
    let vdiv: Node<()> = div(
        vec![],
        vec![svg(
            vec![xmlns("http://www.w3.org/2000/svg")],
            vec![circle(vec![cx("50"), cy("50"), r("50")], vec![])],
        )],
    );
    let created_node = simple_program().create_dom_node(&vdiv);
    let div: Element = created_node.unchecked_into();

    assert_eq!(
        &div.inner_html(),
        r#"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="50" cy="50" r="50"></circle></svg>"#
    );
}

#[wasm_bindgen_test]
fn div_with_attributes() {
    let vdiv: Node<()> = div(vec![id("id-here"), class("two classes")], vec![]);

    let created_node = simple_program().create_dom_node(&vdiv);
    let div: Element = created_node.unchecked_into();

    assert_eq!(&div.id(), "id-here");

    assert!(div.class_list().contains("two"));
    assert!(div.class_list().contains("classes"));

    assert_eq!(div.class_list().length(), 2);
}

#[wasm_bindgen_test]
fn click_event() {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let clicked = Rc::new(Cell::new(false));
    let clicked_clone = Rc::clone(&clicked);

    let elem_id = "click-on-div";
    let vdiv: Node<()> = div(
        vec![
            id(elem_id),
            on_click(move |_| {
                console::log_1(&"clicked event called".into());
                clicked_clone.set(true);
            }),
        ],
        vec![],
    );

    let mut simple_program = simple_program();

    simple_program
        .update_dom_with_vdom(vdiv)
        .expect("must not error");

    let click_event = web_sys::MouseEvent::new("click").unwrap();

    let div = document.get_element_by_id(elem_id).unwrap();

    (EventTarget::from(div))
        .dispatch_event(&click_event)
        .unwrap();

    assert_eq!(*clicked, Cell::new(true));
}
