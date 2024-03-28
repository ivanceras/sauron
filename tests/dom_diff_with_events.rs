extern crate log;
use crate::vdom::TreePath;
use sauron::{html::attributes::*, html::events::*, html::*, *};

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn nodes_with_event_should_not_recycle() {
    console_log::init_with_level(log::Level::Trace).ok();

    let f = |_| log::trace!("I'm a div");
    let old: Node<()> = div(
        vec![class("container")],
        vec![div(vec![class("child"), on_click(f)], vec![])],
    );

    let new: Node<()> = div(
        vec![class("container")],
        vec![div(vec![class("child")], vec![])],
    );

    let diff = diff(&old, &new);
    log::info!("{:#?}", diff);
    assert_eq!(
        diff,
        vec![Patch::remove_attributes(
            &"div",
            TreePath::new(vec![0]),
            vec![&on_click(f)]
        )]
    );
}
