#![deny(warnings)]
use sauron::*;
use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

#[test]
fn simple() {
    let html: Node<()> = html::fragment([div([], []), span([], [])]);
    let expected = "<div></div><span></span>";
    assert_eq!(html.render_to_string(), expected);
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_on_client() {
    let mut simple_program = simple_program();
    let input = node! {
        <div id="container">{ some_view() }</div>
    };

    // this is returning a fragement node
    fn some_view() -> Node<()> {
        node! {
            <h1>"Header"</h1>
            <main>"Main content here."</main>
        }
    }

    let expected = input.render_to_string();

    simple_program.set_current_dom(input);

    let container = sauron_core::dom::document()
        .get_element_by_id("container")
        .unwrap();

    assert_eq!(container.outer_html(), expected);
}
