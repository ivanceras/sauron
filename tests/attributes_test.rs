#![deny(warnings)]
use sauron::Node;

use sauron::html::{attributes::*, *};
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

#[test]
fn test_classes() {
    let actual: Node<&'static str> = div(
        vec![classes(["class1", "class2", "big_blue", "circular"])],
        vec![],
    );
    let actual_html = format!("{}", actual);
    let expected: Node<&'static str> =
        div(vec![class("class1 class2 big_blue circular")], vec![]);
    let expected_html = format!("{}", expected);
    assert_eq!(actual_html, expected_html);
}

#[test]
fn test_classes_flag() {
    let actual: Node<&'static str> = div(
        vec![classes_flag([
            ("class1", true),
            ("class2", false),
            ("big_blue", true),
            ("circular", true),
        ])],
        vec![],
    );
    let actual_html = format!("{}", actual);
    let expected: Node<&'static str> =
        div(vec![class("class1 big_blue circular")], vec![]);
    let expected_html = format!("{}", expected);
    assert_eq!(actual_html, expected_html);
}
