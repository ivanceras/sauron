#![deny(warnings)]
use sauron::Node;

use sauron::html::{
    attributes::*,
    *,
};
#[test]
fn test_styles() {
    let actual: Node<&'static str> = div(
        vec![styles([("display", "flex"), ("flex-direction", "row")])],
        vec![],
    );
    let actual_html = format!("{}", actual);
    let expected: Node<&'static str> =
        div(vec![style("display:flex;flex-direction:row;")], vec![]);
    let expected_html = format!("{}", expected);
    assert_eq!(actual_html, expected_html);
}
