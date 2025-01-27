use std::{cell::RefCell, rc::Rc};

use sauron_core::{
    dom::{self, DomNode},
    prelude::Node,
    vdom,
};
use sauron_html_parser::parse_html;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    log::info!("Run VDOM patch example");

    let ev_callback = |_| {};

    let body_node = DomNode::from(web_sys::Node::from(dom::util::body()));
    let mount_node = Rc::new(RefCell::new(Some(body_node)));

    let new_html = r#"
      <article class="some-class">
        <div id = "my-hello-world-id">
            Hello world!
            If you can see this text on the webpage then the DOM patching was executed!
        </div>
        <footer>This is a footer</footer>
      </article>"#;

    let old_node: Node<()> = parse_html::<()>("").unwrap().unwrap();
    let new_node: Node<()> = parse_html::<()>(new_html).unwrap().unwrap();

    let root = dom::create_dom_node(&old_node, ev_callback);
    let root_node = Rc::new(RefCell::new(Some(root)));

    let vdom_patches = vdom::diff(&old_node, &new_node).unwrap();
    log::info!("Created {} VDOM patch(es)", vdom_patches.len());
    log::debug!("VDOM patch(es): {vdom_patches:?}");

    // convert vdom patch to real dom patches
    let dom_patches = dom::convert_patches(
        &root_node.borrow().as_ref().unwrap(),
        &vdom_patches,
        ev_callback,
    )
    .unwrap();
    log::info!("Converted {} DOM patch(es)", dom_patches.len());
    log::debug!("DOM patch(es): {dom_patches:?}");

    dom::apply_dom_patches(root_node, mount_node, dom_patches).unwrap();
}
