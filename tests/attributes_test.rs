#![deny(warnings)]
use sauron::{Attribute, Node, Render};

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
        vec![attr("style", "display:flex;flex-direction:row;")],
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

#[test]
fn classes_test() {
    let html: Node<()> = div(vec![classes(["class1", "class2"])], vec![]);
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
    let elm = html.as_element_ref().expect("expecting an element");

    let classes: &Attribute<()> = elm
        .get_attributes()
        .into_iter()
        .find(|att| att.name() == &"class")
        .unwrap();

    assert_eq!(
        classes,
        &Attribute::with_multiple_values(
            None,
            "class",
            vec![
                AttributeValue::from_value("class1".to_string().into()),
                AttributeValue::from_value("class2".to_string().into())
            ]
        )
    );
}

#[test]
fn should_merge_classes_flag() {
    let html: Node<()> = div(
        vec![classes_flag([("class1", true), ("class2", true)])],
        vec![],
    );
    let attrs = html.get_attributes().unwrap();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
    let elm = html.as_element_ref().expect("expecting an element");

    let classes: &Attribute<()> = elm
        .get_attributes()
        .into_iter()
        .find(|att| att.name() == &"class")
        .unwrap();

    assert_eq!(
        classes,
        &Attribute::with_multiple_values(
            None,
            "class",
            vec![
                AttributeValue::from_value("class1".to_string().into()),
                AttributeValue::from_value("class2".to_string().into())
            ]
        )
    );
}
