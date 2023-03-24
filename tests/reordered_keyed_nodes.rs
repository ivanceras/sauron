#![deny(warnings)]
use sauron::prelude::*;
use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn failing_reordered_keys() {
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

    let simple_program = simple_program();
    simple_program.set_current_dom(old);

    simple_program.update_dom(update1).await.expect("must not error");

    let container = document
        .query_selector(".reordered")
        .expect("must not error")
        .expect("must exist");

    let expected1 = "<main class=\"reordered\"><ul class=\"todo\"><li key=\"3\">item3</li><li key=\"2\">item2</li><li key=\"1\">item1</li></ul></main>";
    assert_eq!(expected1, container.outer_html());
}
