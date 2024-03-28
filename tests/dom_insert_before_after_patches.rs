#![deny(warnings)]
use sauron::{
    html::{attributes::*, *},
    *,
};
use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn insert_multiple_before_nodes() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("before_nodes_test1")],
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
        vec![class("before_nodes_test1")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![], vec![text("itemA")]),
                li(vec![], vec![text("itemB")]),
                li(vec![], vec![text("itemC")]),
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
            ],
        )],
    );

    let patches = diff(&old, &update1);
    log::debug!("patches: {:#?}", patches);

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");

    let mut simple_program = simple_program();

    simple_program
        .update_dom_with_vdom(old)
        .expect("must update dom");

    let container = document
        .query_selector(".before_nodes_test1")
        .expect("must not error")
        .expect("must exist");

    let expected = "<main class=\"before_nodes_test1\"><ul class=\"todo\"><li key=\"1\">item1</li><li key=\"2\">item2</li><li key=\"3\">item3</li></ul></main>";
    assert_eq!(expected, container.outer_html());

    simple_program
        .update_dom_with_vdom(update1)
        .expect("must not error");

    let container = document
        .query_selector(".before_nodes_test1")
        .expect("must not error")
        .expect("must exist");

    let expected1 = "<main class=\"before_nodes_test1\"><ul class=\"todo\"><li>itemA</li><li>itemB</li><li>itemC</li><li key=\"1\">item1</li><li key=\"2\">item2</li><li key=\"3\">item3</li></ul></main>";

    assert_eq!(expected1, container.outer_html());
}

#[wasm_bindgen_test]
fn insert_multiple_after_nodes() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("after_nodes_test1")],
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
        vec![class("after_nodes_test1")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
                li(vec![], vec![text("itemA")]),
                li(vec![], vec![text("itemB")]),
                li(vec![], vec![text("itemC")]),
            ],
        )],
    );

    let patches = diff(&old, &update1);
    log::debug!("patches: {:#?}", patches);

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");

    let mut simple_program = simple_program();
    simple_program
        .update_dom_with_vdom(old)
        .expect("must update dom");

    let container = document
        .query_selector(".after_nodes_test1")
        .expect("must not error")
        .expect("must exist");

    let expected = "<main class=\"after_nodes_test1\"><ul class=\"todo\"><li key=\"1\">item1</li><li key=\"2\">item2</li><li key=\"3\">item3</li></ul></main>";

    assert_eq!(expected, container.outer_html());

    simple_program
        .update_dom_with_vdom(update1)
        .expect("must not error");

    let container = document
        .query_selector(".after_nodes_test1")
        .expect("must not error")
        .expect("must exist");

    let expected1 = "<main class=\"after_nodes_test1\"><ul class=\"todo\"><li key=\"1\">item1</li><li key=\"2\">item2</li><li key=\"3\">item3</li><li>itemA</li><li>itemB</li><li>itemC</li></ul></main>";

    assert_eq!(expected1, container.outer_html());
}

#[wasm_bindgen_test]
fn insert_multiple_in_the_middle() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        [class("middle_nodes_test1")],
        vec![ul(
            [class("todo")],
            vec![
                li([key(1)], vec![text("item1")]),
                li([key(2)], vec![text("item2")]),
                li([key(3)], vec![text("item3")]),
            ],
        )],
    );

    let update1: Node<()> = main(
        [class("middle_nodes_test1")],
        vec![ul(
            [class("todo")],
            vec![
                li([], vec![text("itemA")]),
                li([], vec![text("itemB")]),
                li([key(1)], vec![text("item1")]),
                li([key(2)], vec![text("item2")]),
                li([key(3)], vec![text("item3")]),
                li([], vec![text("itemC")]),
            ],
        )],
    );

    let patches = diff(&old, &update1);
    log::debug!("patching old to update1: {:#?}", patches);

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");

    let mut simple_program = simple_program();
    simple_program
        .update_dom_with_vdom(old)
        .expect("must update dom");

    let container = document
        .query_selector(".middle_nodes_test1")
        .expect("must not error")
        .expect("must exist");

    let expected = "<main class=\"middle_nodes_test1\">\
        <ul class=\"todo\">\
            <li key=\"1\">item1</li>\
            <li key=\"2\">item2</li>\
            <li key=\"3\">item3</li>\
        </ul>\
    </main>";

    assert_eq!(expected, container.outer_html());
    {
        let root_node = simple_program.root_node.borrow();
        let root_node = root_node.as_ref().unwrap();
        log::info!("root node: {}", root_node.render_to_string());
        assert_eq!(container.outer_html(), root_node.render_to_string());
    }

    simple_program
        .update_dom_with_vdom(update1)
        .expect("must not error");

    let container = document
        .query_selector(".middle_nodes_test1")
        .expect("must not error")
        .expect("must exist");

    let expected1 = "<main class=\"middle_nodes_test1\">\
        <ul class=\"todo\">\
            <li>itemA</li>\
            <li>itemB</li>\
            <li key=\"1\">item1</li>\
            <li key=\"2\">item2</li>\
            <li key=\"3\">item3</li>\
            <li>itemC</li>\
        </ul>\
    </main>";

    assert_eq!(expected1, container.outer_html());

    {
        let root_node = simple_program.root_node.borrow();
        let root_node = root_node.as_ref().unwrap();
        log::info!("The root node is: {}", root_node.render_to_string());
        assert_eq!(container.outer_html(), root_node.render_to_string());
    }
}
