use log::*;
use sauron::{
    html::{attributes::*, events::*, *},
    test_fixtures::simple_program,
    *,
};
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen_test::*;

use web_sys::InputEvent;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn node_mounted_properly() {
    console_log::init_with_level(log::Level::Trace);
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("container1")],
        vec![section(
            vec![class("todo")],
            vec![
                article(vec![key(1)], vec![text("item1")]),
                article(vec![key(2)], vec![text("item2")]),
            ],
        )],
    );

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");
    debug!("old html: {}", old_html);

    let simple_program = simple_program();
    let mut dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, old, &sauron::body());

    let container = document
        .query_selector(".container1")
        .expect("must not error")
        .expect("must exist");

    let expected_outer = "<main class=\"container1\" node_idx=\"0\">\
        <section class=\"todo\" node_idx=\"1\">\
        <article key=\"1\" node_idx=\"2\">item1</article>\
        <article key=\"2\" node_idx=\"4\">item2</article>\
        </section>\
        </main>";

    debug!("original outer html: {}", container.outer_html());
    assert_eq!(expected_outer, container.outer_html());
}

#[wasm_bindgen_test]
fn node_patched_properly() {
    console_log::init_with_level(log::Level::Trace);
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("container2")],
        vec![section(
            vec![class("todo")],
            vec![
                article(vec![key(1)], vec![text("item1")]),
                article(vec![key(2)], vec![text("item2")]),
            ],
        )],
    );

    // we remove the key2
    let new: Node<()> = main(
        vec![class("container2")],
        vec![section(
            vec![class("todo")],
            vec![article(vec![key(1)], vec![text("item1")])],
        )],
    );

    let patches = diff(&old, &new);
    debug!("patches: {:#?}", patches);

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");
    debug!("old html: {}", old_html);

    let simple_program = simple_program();
    let mut dom_updater =
        DomUpdater::new_append_to_mount(&simple_program, old, &sauron::body());

    let container = document
        .query_selector(".container2")
        .expect("must not error")
        .expect("must exist");

    let expected_outer = r#"<main class="container2" node_idx="0"><section class="todo" node_idx="1"><article key="1" node_idx="2">item1</article><article key="2" node_idx="4">item2</article></section></main>"#;

    debug!("original outer html: {}", container.outer_html());
    assert_eq!(expected_outer, container.outer_html());

    dom_updater.update_dom(&simple_program, new);

    let container = document
        .query_selector(".container2")
        .expect("must not error")
        .expect("must exist");
    debug!("after update: {}", container.outer_html());
}
