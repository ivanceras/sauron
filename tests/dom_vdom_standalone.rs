use std::{cell::RefCell, rc::Rc};

use wasm_bindgen_test::*;
use web_sys::{Element, HtmlElement};

use sauron_core::{
    dom::{self, DomNode},
    prelude::Node,
    vdom,
};
use sauron_html_parser::{parse_html, raw_html};

wasm_bindgen_test_configure!(run_in_browser);

// Verify that our DomUpdater's patch method works.
// We test a simple case here, since diff_patch.rs is responsible for testing more complex
// diffing and patching.
#[wasm_bindgen_test]
fn test_dom_vdom_standalone() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    let ev_callback = |_| {};

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let div: Element = document.create_element("div").unwrap();
    div.set_attribute("id", "here").unwrap();
    document.body().unwrap().append_child(&div).unwrap();

    let web_sys_node: web_sys::Node = web_sys::Node::from(div);
    let div_node = DomNode::from(web_sys_node);
    let mount_node = Rc::new(RefCell::new(Some(div_node)));

    let new_html = r#"
    <div>boak</div>
    "#;

    let old_node: Node<()> = parse_html::<()>("").unwrap().unwrap();
    let new_node: Node<()> = raw_html::<()>(new_html);
    let root = dom::create_dom_node(&old_node, ev_callback);
    let root_node = Rc::new(RefCell::new(Some(root)));

    let vdom_patches = vdom::diff(&old_node, &new_node);
    log::debug!("Created {} VDOM patch(es)", vdom_patches.len());
    log::debug!("Created {:?}", vdom_patches);

    // convert vdom patch to real dom patches
    let dom_patches = dom::convert_patches(
        &root_node.borrow().as_ref().unwrap(),
        &vdom_patches,
        ev_callback,
    )
    .unwrap();
    log::debug!("Converted {} DOM patch(es)", dom_patches.len());
    log::debug!("Converted {:?}", dom_patches);

    dom::apply_dom_patches(root_node, mount_node, dom_patches).unwrap();

    let target: Element = document.get_element_by_id("here").unwrap();

    // Get the inner HTML from the body element
    let html_content: String = target.inner_html();

    assert_eq!("<div>boak</div>".to_string(), html_content);
}
