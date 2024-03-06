//#![deny(warnings)]
use sauron::html::attributes::styles;
use sauron::*;

#[test]
fn test_style_macro() {
    let actual: Node<&'static str> =
        div(vec![style! {display:"flex",flex_direction:"row"}], vec![]);
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
fn test_styles() {
    let actual: Node<&'static str> = div(
        vec![html::attributes::styles([
            ("display", "flex"),
            ("flex-direction", "row"),
        ])],
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
    let expected: Node<&'static str> = div(vec![class("class1 class2 big_blue circular")], vec![]);

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
    let expected: Node<&'static str> = div(vec![class("class1 big_blue circular")], vec![]);
    let mut expected_html = String::new();
    expected.render(&mut expected_html).unwrap();

    assert_eq!(actual_html, expected_html);
}

#[test]
fn test_styles_flag() {
    let actual: Node<&'static str> = div(
        vec![styles_flag([
            ("font-family", "monospace", true),
            ("user-select", "none", false),
        ])],
        vec![],
    );
    let mut actual_html = String::new();
    actual.render(&mut actual_html).unwrap();
    let expected: Node<&'static str> = div(vec![style! {"font-family": "monospace"}], vec![]);
    let mut expected_html = String::new();
    expected.render(&mut expected_html).unwrap();

    assert_eq!(actual_html, expected_html);
}

#[test]
fn test_styles_and_styles_flag() {
    /*
    let actual: Node<&'static str> = div(
        vec![
            styles_flag([
                ("font-family", "monospace", true),
                ("user-select", "none", false),
            ]),
            styles([("display", "flex")]),
        ],
        vec![],
    );
    let mut actual_html = String::new();
    actual.render(&mut actual_html).unwrap();
    */


    let s:Attribute<&'static str> = style! {"font-family": "monospace"};
    println!("a style generates: {:#?}", s);

    let expected: Node<&'static str> = div(
        vec![
            style! {"font-family": "monospace"},
            style! {"display": "flex"},
        ],
        vec![],
    );
    println!("expected node: {:#?}", expected);

    let mut expected_html = String::new();
    expected.render(&mut expected_html).unwrap();
    
    println!("expected: {}", expected_html);
    //println!("actual:   {}", actual_html);

    //assert_eq!(actual_html, expected_html);
    panic!();
}

#[test]
fn classes_test() {
    let html: Node<()> = div(vec![classes(["class1", "class2"])], vec![]);
    let attrs = html.attributes().unwrap();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
    let elm = html.element_ref().expect("expecting an element");

    let classes: &Attribute<()> = elm
        .attributes()
        .into_iter()
        .find(|att| att.name() == &"class")
        .unwrap();

    assert_eq!(
        classes,
        &Attribute::with_multiple_values(
            None,
            "class",
            vec![
                AttributeValue::from("class1".to_string()),
                AttributeValue::from("class2".to_string())
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
    let attrs = html.attributes().unwrap();
    println!("attrs: {:#?}", attrs);
    assert_eq!(attrs.len(), 1);
    let elm = html.element_ref().expect("expecting an element");

    let classes: &Attribute<()> = elm
        .attributes()
        .into_iter()
        .find(|att| att.name() == &"class")
        .unwrap();

    assert_eq!(
        classes,
        &Attribute::with_multiple_values(
            None,
            "class",
            vec![
                AttributeValue::from("class1".to_string()),
                AttributeValue::from("class2".to_string())
            ]
        )
    );
}
