#![allow(unused)]
use sauron::dom::DomNode;
use sauron::*;
use test_fixtures::simple_program;
use wasm_bindgen_test::*;
use sauron::parse_html;

mod test_fixtures;

#[test]
fn simple() {
    let html: Node<()> = ul([], [parse_html("<li>Hi</li><li>Hello</li>").unwrap().unwrap()]);
    let expected = "<ul><li>Hi</li><li>Hello</li></ul>";
    assert_eq!(html.render_to_string(), expected);
}

wasm_bindgen_test_configure!(run_in_browser);

//#[wasm_bindgen_test]
fn same_node() {
    let old: Node<()> = ul([], [parse_html("<li>Hi</li><li>Hello</li>").unwrap().unwrap()]);
    let new: Node<()> = ul([], [li([], [text("Hi")]), li([], [text("Hello")])]);
    let simple_program = simple_program();
    let old_node = simple_program.create_dom_node(None, &old);
    let new_node = simple_program.create_dom_node(None, &new);
    log::info!("old_node: {}", old_node.render_to_string());
    assert_eq!(old_node.render_to_string(), new_node.render_to_string());
}
