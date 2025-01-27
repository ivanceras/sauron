use std::{cell::RefCell, rc::Rc};
use wasm_bindgen_test::*;

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

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let div: Element = document.create_element("div").unwrap();
    div.set_attribute("id", "here").unwrap();
    document.body().unwrap().append_child(&div).unwrap();

    let web_sys_node: web_sys::Node = web_sys::Node::from(div);
    let div_node = DomNode::from(web_sys_node);

    let new_html = r#"
    <div>boak</div>
    "#;

    let old_node: Node<()> = parse_html::<()>("").unwrap().unwrap();
    let new_node: Node<()> = raw_html::<()>(new_html);

    let vdom_patches = vdom::diff(&old_node, &new_node).unwrap();
    log::debug!("Created {} VDOM patch(es)", vdom_patches.len());
    log::debug!("Created {:?}", vdom_patches);

    let ev_callback = |_| {};
    let root: DomNode = dom::create_dom_node(&old_node, ev_callback);
    let root_node: Rc<RefCell<Option<DomNode>>> = Rc::new(RefCell::new(Some(root)));

    let dom_patches = dom::convert_patches(
        &root_node.borrow().as_ref().unwrap(),
        &vdom_patches,
        ev_callback,
    )
    .unwrap();
    log::debug!("Converted {} DOM patch(es)", dom_patches.len());
    log::debug!("Converted {:?}", dom_patches);

    let mount_node: Rc<RefCell<Option<DomNode>>> = Rc::new(RefCell::new(Some(div_node)));
    dom::apply_dom_patches(root_node, mount_node, dom_patches).unwrap();

    let target: Element = document.get_element_by_id("here").unwrap();
    let html_content: String = target.inner_html();

    assert_eq!("<div>boak</div>".to_string(), html_content);
}


#[derive(Clone)]
struct DomUpdater {
    id: String,
    current_vdom: Node<()>,
    root_node: Rc<RefCell<Option<DomNode>>>,
    mount_node: Rc<RefCell<Option<DomNode>>>,
}

impl DomUpdater {
    fn new(id: String) -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let div: web_sys::Element = document.create_element("div").unwrap();
        div.set_attribute("id", id.as_str()).unwrap();
        document.body().unwrap().append_child(&div).unwrap();

        let web_sys_node: web_sys::Node = web_sys::Node::from(div);
        let div_node = DomNode::from(web_sys_node);

        let current_vdom: Node<()> = parse_html::<()>("").unwrap().unwrap();
        let ev_callback = |_| {};
        let root: DomNode = dom::create_dom_node(&current_vdom, ev_callback);

        DomUpdater {
            id,
            current_vdom,
            root_node: Rc::new(RefCell::new(Some(root))),
            mount_node: Rc::new(RefCell::new(Some(div_node))),
        }
    }
    fn update(&mut self, next_html: String) {
        let new_node: Node<()> = parse_html::<()>(next_html.as_str()).unwrap().unwrap();

        let old_vdom = self.current_vdom.clone();

        log::debug!("-------------------------------------------------");
        log::debug!("old_node: {}", old_vdom.render_to_string());
        log::debug!("inner_html: {}", self.inner_html());
        fn same(a: String, b: String) -> String {
            if a == b {
                "same".to_string()
            } else {
                "different".to_string()
            }
        }
        log::debug!(
            "   => {}",
            same(old_vdom.render_to_string(), self.inner_html())
        );
        log::debug!("new_node: {}", new_node.render_to_string());
        log::debug!("new_node: {:#?}", new_node);

        let vdom_patches = vdom::diff(&old_vdom, &new_node).unwrap();

        log::debug!("Created {} VDOM patch(es)", vdom_patches.len());
        log::debug!("Created {:#?}", vdom_patches);
        let dom_patches = dom::convert_patches(
            self.root_node.borrow().as_ref().unwrap(),
            &vdom_patches,
            |_| {},
        )
        .unwrap();
        log::debug!("Converted {} DOM patch(es)", dom_patches.len());
        log::debug!("Converted {:#?}", dom_patches);
        log::debug!("-------------------------------------------------");
        dom::apply_dom_patches(
            Rc::clone(&self.root_node),
            Rc::clone(&self.mount_node),
            dom_patches,
        )
        .unwrap();
        self.current_vdom = new_node.clone();

        assert_eq!(next_html, self.inner_html());
    }
    fn inner_html(&self) -> String {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let target: Element = document.get_element_by_id(self.id.as_str()).unwrap();
        target.inner_html()
    }
}

#[wasm_bindgen_test]
fn test_dom_vdom_patcher() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();
    let id: String = "there".to_string();

    let mut dom_updater: DomUpdater = DomUpdater::new(id.clone());

    let html: String = "<div id=\"there\"></div>".to_string();
    dom_updater.update(html.clone());
    assert_eq!(html.to_string(), dom_updater.inner_html());

    let html: String = "<div></div>".to_string();
    dom_updater.update(html.clone());
    assert_eq!(html, dom_updater.inner_html());

    let html: String = "<div id=\"there\"><b>foo</b></div>".to_string();
    dom_updater.update(html.clone());

    assert_eq!(html, dom_updater.inner_html());
}
