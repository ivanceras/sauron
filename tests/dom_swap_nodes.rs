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
fn swap_rows_non_keyed() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let class_name = "swap_rows_non_keyed";

    let old: Node<()> = main(
        vec![class(class_name)],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![class(1)], vec![text("item1")]),
                li(vec![class(2)], vec![text("item2")]),
                li(vec![class(3)], vec![text("item3")]),
                li(vec![class(4)], vec![text("item4")]),
                li(vec![class(5)], vec![text("item5")]),
            ],
        )],
    );

    let update1: Node<()> = main(
        vec![class(class_name)],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![class(1)], vec![text("item1")]),
                li(vec![class(4)], vec![text("item4")]),
                li(vec![class(3)], vec![text("item3")]),
                li(vec![class(2)], vec![text("item2")]),
                li(vec![class(5)], vec![text("item5")]),
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

    let expected1 = update1.render_to_string();

    simple_program
        .update_dom_with_vdom(update1)
        .expect("must not error");

    let container = document
        .query_selector(&format!(".{class_name}"))
        .expect("must not error")
        .expect("must exist");

    assert_eq!(expected1, container.outer_html());
}

#[wasm_bindgen_test]
fn swap_rows_keyed() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
    let class_name = "swap_rows_keyed";

    let old: Node<()> = main(
        vec![class(class_name)],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
                li(vec![key(4)], vec![text("item4")]),
                li(vec![key(5)], vec![text("item5")]),
            ],
        )],
    );

    let update1: Node<()> = main(
        vec![class(class_name)],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(4)], vec![text("item4")]),
                li(vec![key(3)], vec![text("item3")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(5)], vec![text("item5")]),
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

    let expected1 = update1.render_to_string();

    simple_program
        .update_dom_with_vdom(update1)
        .expect("must not error");

    let container = document
        .query_selector(&format!(".{class_name}"))
        .expect("must not error")
        .expect("must exist");

    assert_eq!(expected1, container.outer_html());
}

#[wasm_bindgen_test]
fn swap_1_and_8() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
    let class_name = "swap_8th";

    let old: Node<()> = main(
        vec![class(class_name)],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(0)], vec![text("item0")]),
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
                li(vec![key(4)], vec![text("item4")]),
                li(vec![key(5)], vec![text("item5")]),
                li(vec![key(6)], vec![text("item6")]),
                li(vec![key(7)], vec![text("item7")]),
                li(vec![key(8)], vec![text("item8")]),
                li(vec![key(9)], vec![text("item9")]),
            ],
        )],
    );

    let update1: Node<()> = main(
        vec![class(class_name)],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(0)], vec![text("item0")]),
                li(vec![key(8)], vec![text("item8")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(3)], vec![text("item3")]),
                li(vec![key(4)], vec![text("item4")]),
                li(vec![key(5)], vec![text("item5")]),
                li(vec![key(6)], vec![text("item6")]),
                li(vec![key(7)], vec![text("item7")]),
                li(vec![key(1)], vec![text("item1")]),
                li(vec![key(9)], vec![text("item9")]),
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

    let expected1 = update1.render_to_string();

    simple_program
        .update_dom_with_vdom(update1)
        .expect("must not error");

    let container = document
        .query_selector(&format!(".{class_name}"))
        .expect("must not error")
        .expect("must exist");

    assert_eq!(expected1, container.outer_html());
}
