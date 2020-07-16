#![deny(warnings)]
use sauron::{Node, Render};

use sauron::html::{attributes::*, *};
#[test]
fn test_styles() {
    let actual: Node<&'static str> = div(
        vec![styles([("display", "flex"), ("flex-direction", "row")])],
        vec![],
    );
    let mut actual_html = String::new();
    actual.render(&mut actual_html).unwrap();
    let expected: Node<&'static str> = div(
        vec![style("display", "flex"), style("flex-direction", "row")],
        vec![],
    );
    let mut expected_html = String::new();
    expected.render(&mut expected_html).unwrap();

    assert_eq!(actual_html, expected_html);
}

#[test]
fn test_classes() {
    let actual: Node<&'static str> = div(
        vec![classes(["class1", "class2", "big_blue", "circular"])],
        vec![],
    );
    let mut actual_html = String::new();
    actual.render(&mut actual_html).unwrap();
    let expected: Node<&'static str> =
        div(vec![class("class1 class2 big_blue circular")], vec![]);

    let mut expected_html = String::new();
    expected.render(&mut expected_html).unwrap();

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
    let mut actual_html = String::new();
    actual.render(&mut actual_html).unwrap();
    let expected: Node<&'static str> =
        div(vec![class("class1 big_blue circular")], vec![]);
    let mut expected_html = String::new();
    expected.render(&mut expected_html).unwrap();

    assert_eq!(actual_html, expected_html);
}
