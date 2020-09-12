use log::*;
use sauron_core::{
    html::{attributes::*, events::*, *},
    mt_dom::diff::ChangeText,
    *,
};
use std::{cell::RefCell, rc::Rc};
use test_fixtures::simple_program;
use wasm_bindgen_test::*;
use web_sys::InputEvent;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

// Issue: When there is diff_keyed_elements
// the first update is OK, however, the subsequent update
// will error with:
//
// : panicked at 'must have a tag here',
// sauron/crates/sauron-core/src/dom/apply_patches.rs:109:32

#[wasm_bindgen_test]
fn subsequent_updates() {
    console_log::init_with_level(log::Level::Trace);
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("test5")],
        vec![
            section(
                vec![class("todo")],
                vec![
                    article(vec![key(1)], vec![text("item1")]),
                    article(vec![key(2)], vec![text("item2")]),
                    article(vec![key(3)], vec![text("item3")]),
                ],
            ),
            footer(vec![], vec![text("3 items left")]),
        ],
    );

    // we remove the key1
    let update1: Node<()> = main(
        vec![class("test5")],
        vec![
            section(
                vec![class("todo")],
                vec![
                    article(vec![key(2)], vec![text("item2")]),
                    article(vec![key(3)], vec![text("item3 with changes")]),
                ],
            ),
            footer(vec![], vec![text("2 items left")]),
        ],
    );

    let update1_clone = update1.clone();

    let patches = diff(&old, &update1);
    assert_eq!(
        patches,
        vec![
            Patch::ChangeText(ChangeText::new(
                7,
                "item3",
                "item3 with changes"
            )),
            Patch::RemoveChildren(&"section", 1, vec![0]),
            Patch::ChangeText(ChangeText::new(
                9,
                "3 items left",
                "2 items left"
            ))
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
        .query_selector(".test5")
        .expect("must not error")
        .expect("must exist");

    let expected = "<main class=\"test5\">\
        <section class=\"todo\">\
        <article key=\"1\">item1</article>\
        <article key=\"2\">item2</article>\
        <article key=\"3\">item3</article>\
        </section>\
        <footer>3 items left</footer>\
        </main>";

    assert_eq!(expected, container.outer_html());

    dom_updater.update_dom(&simple_program, update1);

    let container = document
        .query_selector(".test5")
        .expect("must not error")
        .expect("must exist");

    let expected1 = "<main class=\"test5\">\
        <section class=\"todo\">\
        <article key=\"2\">item2</article>\
        <article key=\"3\">item3 with changes</article>\
        </section>\
        <footer>2 items left</footer>\
        </main>";

    assert_eq!(expected1, container.outer_html());

    let update2: Node<()> = main(
        vec![class("test5")],
        vec![
            section(
                vec![class("todo")],
                vec![
                    article(vec![key(2)], vec![text("item2")]),
                    article(vec![key(3)], vec![text("item3 with changes")]),
                    article(vec![key(4)], vec![text("Added a new key4")]),
                ],
            ),
            footer(vec![], vec![text("2 items left")]),
        ],
    );

    let patches2 = diff(&update1_clone, &update2);
    assert_eq!(
        patches2,
        vec![Patch::AppendChildren(
            &"section",
            1,
            vec![&article(vec![key(4)], vec![text("Added a new key4")])]
        )]
    );

    dom_updater.update_dom(&simple_program, update2);

    let expected2 = "<main class=\"test5\">\
        <section class=\"todo\">\
        <article key=\"2\">item2</article>\
        <article key=\"3\">item3 with changes</article>\
        <article key=\"4\">Added a new key4</article>\
        </section>\
        <footer>2 items left</footer>\
        </main>";

    assert_eq!(expected2, container.outer_html());
}
