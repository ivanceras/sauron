#![deny(warnings)]
use sauron::prelude::*;
use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;
#[wasm_bindgen_test]
async fn reordered_keys2() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("reordered2")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
            ],
        )],
    );

    let update1: Node<()> = main(
        vec![class("reordered2")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
                li(vec![key(1)], vec![text("item1")]),
            ],
        )],
    );

    let patches = diff(&old, &update1);
    log::debug!("patches: {:#?}", patches);

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");

    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        old,
        &sauron_core::body(),
    );

    let update1_html = update1.render_to_string();
    dom_updater.update_dom(&simple_program, update1).await;

    let container = document
        .query_selector(".reordered2")
        .expect("must not error")
        .expect("must exist");

    let expected1 = "<main class=\"reordered2\"><ul class=\"todo\"><li key=\"2\">item2</li><li key=\"3\">item3</li><li key=\"1\">item1</li></ul></main>";
    //FIXME: it reflect the new update
    assert_eq!(expected1, container.outer_html());
    assert_eq!(update1_html, container.outer_html());
}

#[wasm_bindgen_test]
async fn reordered_keys3() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("reordered3")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
            ],
        )],
    );

    let update1: Node<()> = main(
        vec![class("reordered3")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(3)], vec![text("item3")]),
            ],
        )],
    );

    let patches = diff(&old, &update1);
    log::debug!("patches: {:#?}", patches);

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");

    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        old,
        &sauron_core::body(),
    );

    let update1_html = update1.render_to_string();
    dom_updater.update_dom(&simple_program, update1).await;

    let container = document
        .query_selector(".reordered3")
        .expect("must not error")
        .expect("must exist");

    let expected1 = "<main class=\"reordered3\"><ul class=\"todo\"><li key=\"2\">item2</li><li key=\"1\">item1</li><li key=\"3\">item3</li></ul></main>";
    //FIXME: it reflect the new update
    assert_eq!(expected1, container.outer_html());
    assert_eq!(update1_html, container.outer_html());
}
