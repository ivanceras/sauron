#![deny(warnings)]
use sauron::prelude::*;

#[test]
fn simple() {
    let html: Node<()> = fragment([div([], []), span([], [])]);
    let expected = "<div></div><span></span>";
    assert_eq!(html.render_to_string(), expected);
}
