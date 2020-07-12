#![deny(warnings)]
use sauron::{
    sauron_vdom::Style,
    Attribute,
    Element,
    Node,
    Value,
};

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
    let expected: Node<&'static str> = div(
        vec![style("display", "flex"), style("flex-direction", "row")],
        vec![],
    );
    let expected_html = format!("{}", expected);
    assert_eq!(actual_html, expected_html);
}

#[test]
fn test_style_aggregate() {
    let mut elm: Element<&'static str> = Element::with_tag("div");
    elm.add_style("display", "flex");
    elm.add_style("flex-direction", "row");
    elm.add_attributes(vec![attr("width", "100%")]);
    let att = elm.aggregate_styles().unwrap();
    println!("att: {:?}", att);

    assert_eq!(
        att,
        Attribute::from_styles(vec![
            Style {
                name: "display",
                value: Value::Str("flex")
            },
            Style {
                name: "flex-direction",
                value: Value::Str("row")
            }
        ])
    );
    let node: Node<&'static str> = elm.into();

    println!("html: {}", node.to_string());
    let expected =
        r#"<div width="100%" style="display:flex;flex-direction:row;"></div>"#;
    assert_eq!(expected, node.to_string());
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
