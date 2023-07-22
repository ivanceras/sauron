use sauron::{
    html::{attributes::*, *},
    *,
};

#[test]
fn style_should_be_valid() {
    let node1: Node<()> = node!(<div style="border:1px solid green;"></div>);
    let expected = r#"<div style="border:1px solid green;"></div>"#;
    let mut buffer = String::new();
    node1.render(&mut buffer).expect("must have no error");
    assert_eq!(expected, buffer);
}

#[test]
fn with_events() {
    let result: Node<()> = node! {
        <div id="hello" on_click=|_|{println!("clicked!")} >Hello world</div>
    };
    let expected = "<div id=\"hello\" >Hello world</div>";
    assert_eq!(expected, result.render_to_string());
}

#[test]
fn unquoted_text() {
    let result: Node<()> = node! {
        <div id="hello">Hello world</div>
    };

    let expected: Node<()> = div(vec![id("hello")], vec![text("Hello world")]);
    assert_eq!(expected, result);
}

#[test]
fn quoted_text() {
    let result: Node<()> = node! {
        <div id="hello">"Hello world"</div>
    };

    let expected: Node<()> = div(vec![id("hello")], vec![text("Hello world")]);
    assert_eq!(expected, result);
}

#[test]
fn must_correctly_create_self_closing_tag() {
    let result: Node<()> = node! {
        <img src="hello.jpg"></img>
    };

    let expected = r#"<img src="hello.jpg"/>"#;
    assert_eq!(expected, result.render_to_string());
}

#[test]
fn must_correctly_create_non_self_closing_tag() {
    let result: Node<()> = node! {
        <div class="hello"></div>
    };

    let expected = r#"<div class="hello"></div>"#;
    assert_eq!(expected, result.render_to_string());
}

#[test]
fn must_correctly_create_tags_with_namespace() {
    let result: Node<()> = node! {
        <svg><rect x="1" y="1"></rect></svg>
    };

    let expected = r#"<svg><rect x="1" y="1"></rect></svg>"#;
    assert_eq!(expected, result.render_to_string());
}
