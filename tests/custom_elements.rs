#![allow(warnings)]
use sauron::prelude::*;
use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen_test]
fn should_only_define_custom_element_onces() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();

    let old: Node<()> = main(
        vec![class("custom_test1")],
        vec![
            custom_element(
                "my-clock",
                [attr("time", "now")],
                [button([], [text("a clock")])],
            ),
            custom_element(
                "my-clock",
                [attr("time", "later")],
                [button([], [text("another clock")])],
            ),
        ],
    );

    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        old,
        &sauron_core::body(),
    );

    let container = sauron::document()
        .query_selector(".custom_test1")
        .expect("must not error")
        .expect("must exist");

    let expected = "<main class=\"custom_test1\"><my-clock time=\"now\"><button>a clock</button></my-clock><my-clock time=\"later\"><button>another clock</button></my-clock></main>";
    assert_eq!(expected, container.outer_html());
}
