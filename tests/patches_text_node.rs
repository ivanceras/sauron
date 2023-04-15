#![deny(warnings)]
use sauron::{mt_dom::TreePath, prelude::*};

use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn patches_text() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("text_container")],
        vec![section(
            vec![class("todo")],
            vec![article(vec![], vec![text("item1")])],
        )],
    );

    let update1: Node<()> = main(
        vec![class("text_container")],
        vec![section(
            vec![class("todo")],
            vec![article(vec![], vec![text("item1 has changed...")])],
        )],
    );

    let patches = diff(&old, &update1);
    log::debug!("patches: {:#?}", patches);
    assert_eq!(
        patches,
        vec![Patch::replace_node(
            None,
            TreePath::new(vec![0, 0, 0]),
            &text("item1 has changed...")
        )]
    );

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");

    let simple_program = simple_program();

    simple_program.set_current_dom(old);

    let container = document
        .query_selector(".text_container")
        .expect("must not error")
        .expect("must exist");

    let expected = "<main class=\"text_container\">\
        <section class=\"todo\">\
        <article>item1 has changed...</article>\
        </section>\
        </main>";

    simple_program.update_dom_with_vdom(update1).expect("must not error");
    let result = container.outer_html();
    log::info!("result: {}", result);
    println!("result: {}", result);
    assert_eq!(expected, result);
}
