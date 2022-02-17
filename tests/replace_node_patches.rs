#![deny(warnings)]
use sauron_core::{
    diff,
    html::{attributes::*, *},
    mt_dom::patch::*,
    DomUpdater, Node, Patch, Render,
};

use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen_test]
fn test_multiple_replace() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("multi_replace")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
            ],
        )],
    );

    // we remove the key1
    let update1: Node<()> = main(
        vec![class("multi_replace")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(10)], vec![text("item10")]),
                li(vec![key(20)], vec![text("item20")]),
                li(vec![key(30)], vec![text("item30")]),
            ],
        )],
    );

    let patches = diff(&old, &update1);
    log::debug!("patches: {:#?}", patches);
    assert_eq!(
        patches,
        vec![
            Patch::replace_node(
                Some(&"li"),
                TreePath::new(vec![0, 0, 0]),
                &li(vec![key(10)], vec![text("item10")]),
            ),
            Patch::replace_node(
                Some(&"li"),
                TreePath::new(vec![0, 0, 1]),
                &li(vec![key(20)], vec![text("item20")]),
            ),
            Patch::replace_node(
                Some(&"li"),
                TreePath::new(vec![0, 0, 2]),
                &li(vec![key(30)], vec![text("item30")]),
            ),
        ]
    );

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");

    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        old,
        &sauron_core::body(),
    );

    let container = document
        .query_selector(".multi_replace")
        .expect("must not error")
        .expect("must exist");

    let expected = "<main class=\"multi_replace\"><ul class=\"todo\"><li key=\"1\">item1</li><li key=\"2\">item2</li><li key=\"3\">item3</li></ul></main>";

    assert_eq!(expected, container.outer_html());

    dom_updater.update_dom(&simple_program, update1);

    let container = document
        .query_selector(".multi_replace")
        .expect("must not error")
        .expect("must exist");

    let expected1 = "<main class=\"multi_replace\"><ul class=\"todo\"><li key=\"10\">item10</li><li key=\"20\">item20</li><li key=\"30\">item30</li></ul></main>";

    assert_eq!(expected1, container.outer_html());
}

#[wasm_bindgen_test]
fn test_multiple_replace_and_parent_is_replaced_too() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("parent_replaced"), key("parent-old")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
            ],
        )],
    );

    // we remove the key1
    let update1: Node<()> = main(
        vec![class("parent_replaced"), key("parent-new")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(10)], vec![text("item10")]),
                li(vec![key(20)], vec![text("item20")]),
                li(vec![key(30)], vec![text("item30")]),
            ],
        )],
    );

    let patches = diff(&old, &update1);
    log::debug!("patches: {:#?}", patches);
    assert_eq!(
        patches,
        vec![Patch::replace_node(
            Some(&"main"),
            TreePath::new(vec![0]),
            &main(
                vec![class("parent_replaced"), key("parent-new")],
                vec![ul(
                    vec![class("todo")],
                    vec![
                        li(vec![key(10)], vec![text("item10")]),
                        li(vec![key(20)], vec![text("item20")]),
                        li(vec![key(30)], vec![text("item30")]),
                    ],
                )],
            )
        )]
    );

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");

    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        old,
        &sauron_core::body(),
    );

    let container = document
        .query_selector(".parent_replaced")
        .expect("must not error")
        .expect("must exist");

    let expected = "<main class=\"parent_replaced\" key=\"parent-old\"><ul class=\"todo\"><li key=\"1\">item1</li><li key=\"2\">item2</li><li key=\"3\">item3</li></ul></main>";

    assert_eq!(expected, container.outer_html());

    dom_updater.update_dom(&simple_program, update1);

    let container = document
        .query_selector(".parent_replaced")
        .expect("must not error")
        .expect("must exist");

    let expected1 = "<main class=\"parent_replaced\" key=\"parent-new\"><ul class=\"todo\"><li key=\"10\">item10</li><li key=\"20\">item20</li><li key=\"30\">item30</li></ul></main>";

    assert_eq!(expected1, container.outer_html());
}
