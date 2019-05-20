use sauron::Node;

use sauron::html::{
    attributes::*,
    *,
};
#[test]
fn test_styles() {
    let actual: Node<&'static str> = div(
        [styles([("display", "flex"), ("flex-direction", "row")])],
        [],
    );
    let actual_html = format!("{}", actual);
    let expected: Node<&'static str> =
        div([style("display:flex;flex-direction:row;")], []);
    let expected_html = format!("{}", expected);
    assert_eq!(actual_html, expected_html);
}
