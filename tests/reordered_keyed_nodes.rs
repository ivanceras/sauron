#![deny(warnings)]
use sauron::*;
use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn failing_reordered_keys() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();

    let old: Node<()> = main(
        vec![class("reordered")],
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
        vec![class("reordered")],
        vec![ul(
            vec![class("todo")],
            vec![
                li(vec![key(3)], vec![text("item3")]),
                li(vec![key(2)], vec![text("item2")]),
                li(vec![key(1)], vec![text("item1")]),
            ],
        )],
    );

    // The patch was:
    //  append key1, item1
    //  remove nodeidx: 2, [0,0,0]
    let patches = diff(&old, &update1);
    log::debug!("patches: {:#?}", patches);

    let mut old_html = String::new();
    old.render(&mut old_html).expect("must render");

    let mut simple_program = simple_program();
    simple_program.set_current_dom(old);

    let expected1 = update1.render_to_string();

    simple_program
        .update_dom_with_vdom(update1)
        .expect("must not error");

    let container = document
        .query_selector(".reordered")
        .expect("must not error")
        .expect("must exist");

    let todos = document
        .query_selector(".reordered .todo")
        .expect("must not error")
        .expect("must exist");

    assert_eq!(todos.child_element_count(), 3);

    assert_eq!(expected1, container.outer_html());
}
